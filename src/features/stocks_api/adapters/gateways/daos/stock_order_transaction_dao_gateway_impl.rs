use std::str::FromStr;

use async_trait::async_trait;
use chrono::DateTime;
use tokio_postgres::Client;

use crate::features::{
    stocks_api::application::interfaces::gateways::daos::stock_order_transaction_dao_gateway::{
        GetWalletHistoricalStatisticsParametersDTO, GetWalletHistoricalStatisticsResultDTO,
        GetWalletHistoricalStatisticsResultItemDTO, GetWalletParametersDTO, GetWalletResultDTO,
        GetWalletResultItemDTO, StockOrderTransactionDAOGateway,
        StockOrderTransactionDAOGatewayConstructor,
    },
    transactions_worker::domain::entities::stock_order_transaction::StockOrderTransactionOperation,
};

struct PreBuiltQueries {
    wallet_query: String,
    wallet_historical_statistics_query: String,
}

pub struct StockOrderTransactionDAOGatewayImpl {
    materialize_client: Client,
    queries: PreBuiltQueries,
}

//  //  //

impl StockOrderTransactionDAOGatewayImpl {}

#[async_trait]
impl StockOrderTransactionDAOGatewayConstructor for StockOrderTransactionDAOGatewayImpl {
    async fn new() -> Self {
        let materialize_url = std::env::var("MATERIALIZE_URL").unwrap();
        let stream_host = std::env::var("KAFKA_BROKER_HOST").unwrap();
        let connection_name = "kafka_connection";

        let stock_order_transaction_source_name = "stock_order_transaction_source";
        let stock_order_transaction_topic_name = "stock_order_transaction";

        let wallet_view_name = "wallet_view";
        let wallet_historical_statistics_view_name = "wallet_historical_statistics_view";

        // Create the connections and get the client
        let materialize_client = {
            // Connect to Materialize
            let (client, connection) =
                tokio_postgres::connect(materialize_url.as_str(), tokio_postgres::NoTls)
                    .await
                    .expect("Failed to connect to Materialize");

            tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("connection error: {}", e);
                }
            });

            // Connect Materialize to Kafka
            client
                .execute(
                    &format!(
                        "
                        CREATE CONNECTION IF NOT EXISTS {connection_name} 
                        TO KAFKA (BROKER '{stream_host}')
                    "
                    ),
                    &[],
                )
                .await
                .expect("Failed to establish the connection with Materialize");

            client
        };

        //  //  //

        // Create the sources
        materialize_client
            .execute(
                &format!(
                    "
                    CREATE SOURCE IF NOT EXISTS {stock_order_transaction_source_name} 
                    FROM KAFKA CONNECTION {connection_name} (TOPIC '{stock_order_transaction_topic_name}') 
                    FORMAT JSON WITH (SIZE = 'mem-16')
                "
                ),
                &[],
            )
            .await
            .expect(&format!(
                "Failed to create the Materialize Source: {stock_order_transaction_source_name}"
            ));

        //  //  //

        // https://materialize.com/docs/sql/create-source/#json
        // Create the views and their related queries
        // P.S.: keeping them alongside each other for more cohesion and to facilitate viewing the fields
        let (wallet_materialized_view, wallet_query) = (
                format!(
                    "
                    CREATE OR REPLACE MATERIALIZED VIEW {wallet_view_name} AS
                    SELECT
                        (data->>'user_id')::text AS user_id,
                        (data->>'stock')::text AS stock,
                        SUM(
                            CASE WHEN (data->>'operation')::text = 'PURCHASE' THEN (data->>'shares')::int ELSE 0 END +
                            CASE WHEN (data->>'operation')::text = 'SELL' THEN -(data->>'shares')::int ELSE 0 END
                        ) AS total_shares,
                        SUM(CASE WHEN (data->>'operation')::text = 'PURCHASE' THEN (data->>'price')::double ELSE 0 END) AS total_purchased_value,
                        SUM(CASE WHEN (data->>'operation')::text = 'SELL' THEN (data->>'price')::double ELSE 0 END) AS total_sold_value
                    FROM {stock_order_transaction_source_name}
                    GROUP BY user_id, stock;
                    "
                ),
                format!(
                    "
                    SELECT user_id, stock, total_shares, total_purchased_value, total_sold_value
                    FROM {wallet_view_name}
                    WHERE user_id = $1;
                    "
                )
        );
        materialize_client
            .execute(&wallet_materialized_view, &[])
            .await
            .expect(&format!(
                "Failed to create the Materialize View: {wallet_view_name}"
            ));

        //

        let (wallet_historical_statistics_materialized_view, wallet_historical_statistics_query) = (
            format!(
                "
                CREATE OR REPLACE MATERIALIZED VIEW {wallet_historical_statistics_view_name} AS
                SELECT DISTINCT ON (
                    (data->>'user_id')::text,
                    (data->>'stock')::text,
                    (data->>'operation')::text
                )
                    (data->>'user_id')::text AS user_id,
                    (data->>'stock')::text AS stock,
                    (data->>'operation')::text AS operation,
                    (data->>'date')::text AS first_operation_date,
                    (data->>'price')::double AS first_operation_value
                FROM {stock_order_transaction_source_name}
                ORDER BY
                (data->>'user_id')::text, 
                (data->>'stock')::text, 
                (data->>'operation')::text,
                (data->>'date')::text
                ASC;
                "
            ),
            format!(
                "
                SELECT user_id, stock, operation, first_operation_date, first_operation_value
                FROM {wallet_historical_statistics_view_name}
                WHERE user_id = $1;
                "
            ),
        );
        materialize_client
            .execute(&wallet_historical_statistics_materialized_view, &[])
            .await
            .expect(&format!(
                "Failed to create the Materialize View: {wallet_historical_statistics_view_name}"
            ));

        //

        StockOrderTransactionDAOGatewayImpl {
            materialize_client,
            queries: PreBuiltQueries {
                wallet_query,
                wallet_historical_statistics_query,
            },
        }
    }
}

#[async_trait]
impl StockOrderTransactionDAOGateway for StockOrderTransactionDAOGatewayImpl {
    async fn get_wallet(
        &self,
        params: GetWalletParametersDTO,
    ) -> Result<GetWalletResultDTO, Box<dyn std::error::Error + Send + Sync>> {
        let GetWalletParametersDTO { user_id } = params;

        let query_result = self
            .materialize_client
            .query(&self.queries.wallet_query, &[&user_id])
            .await?;

        let result = GetWalletResultDTO {
            wallet: query_result
                .iter()
                .map(|result| GetWalletResultItemDTO {
                    stock: result.get("stock"),
                    total_shares: result.get::<&str, i64>("total_shares"), // int8 == i64,
                    total_purchased_value: result.get::<&str, f64>("total_purchased_value"), // float8 == f64
                    total_sold_value: result.get::<&str, f64>("total_sold_value"), // float8 == f64
                })
                .collect(),
        };

        Ok(result)
    }

    async fn get_wallet_historical_statistics(
        &self,
        params: GetWalletHistoricalStatisticsParametersDTO,
    ) -> Result<GetWalletHistoricalStatisticsResultDTO, Box<dyn std::error::Error + Send + Sync>>
    {
        let GetWalletHistoricalStatisticsParametersDTO { user_id } = params;

        let query_result = self
            .materialize_client
            .query(
                &self.queries.wallet_historical_statistics_query,
                &[&user_id],
            )
            .await?;

        let result = GetWalletHistoricalStatisticsResultDTO {
            data: query_result
                .iter()
                .map(|result| GetWalletHistoricalStatisticsResultItemDTO {
                    stock: result.get("stock"),
                    operation: StockOrderTransactionOperation::from_str(result.get("operation"))
                        .unwrap(),
                    first_operation_date: DateTime::parse_from_rfc3339(
                        result.get("first_operation_date"),
                    )
                    .unwrap()
                    .into(),
                    first_operation_value: result.get::<&str, f64>("first_operation_value"), // float8 == f64
                })
                .collect(),
        };

        Ok(result)
    }
}
