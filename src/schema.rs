// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct CoaMasterAccountTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct CoaMasterStatusEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct FinancialYearStatusEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct StockLocationsLocationTypeEnum;

    #[derive(diesel::query_builder::QueryId, Clone, diesel::sql_types::SqlType)]
    #[diesel(mysql_type(name = "Enum"))]
    pub struct StockMovementsMovementTypeEnum;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::CoaMasterAccountTypeEnum;
    use super::sql_types::CoaMasterStatusEnum;

    coa_master (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        code -> Varchar,
        #[max_length = 9]
        account_type -> CoaMasterAccountTypeEnum,
        parent_id -> Nullable<Integer>,
        #[max_length = 10]
        currency_code -> Nullable<Varchar>,
        #[max_length = 8]
        status -> Nullable<CoaMasterStatusEnum>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    exchange_rate (id) {
        id -> Integer,
        #[max_length = 10]
        base_currency -> Varchar,
        #[max_length = 10]
        target_currency -> Varchar,
        rate -> Decimal,
        effective_date -> Date,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::FinancialYearStatusEnum;

    financial_year (id) {
        id -> Unsigned<Bigint>,
        #[max_length = 20]
        name -> Varchar,
        start_date -> Date,
        end_date -> Date,
        #[max_length = 6]
        status -> Nullable<FinancialYearStatusEnum>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    inventory_adjustments (id) {
        id -> Integer,
        product_id -> Integer,
        location_id -> Integer,
        old_quantity -> Integer,
        new_quantity -> Integer,
        adjustment_reason -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    journal (id) {
        id -> Integer,
        #[max_length = 50]
        voucher_no -> Varchar,
        ledger_id -> Integer,
        transaction_type_id -> Integer,
        #[max_length = 255]
        transaction_reference -> Nullable<Varchar>,
        transaction_date -> Date,
        description -> Nullable<Text>,
        debit -> Nullable<Decimal>,
        credit -> Nullable<Decimal>,
        #[max_length = 10]
        currency_code -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    ledger (id) {
        id -> Integer,
        coa_id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        code -> Varchar,
        parent_id -> Nullable<Integer>,
        #[max_length = 10]
        currency_code -> Nullable<Varchar>,
        #[max_length = 20]
        financial_year -> Varchar,
        opening_balance -> Nullable<Decimal>,
        closing_balance -> Nullable<Decimal>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    product_categories (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    products (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 50]
        sku -> Varchar,
        category_id -> Integer,
        description -> Nullable<Text>,
        #[max_length = 50]
        unit -> Varchar,
        cost_price -> Decimal,
        selling_price -> Decimal,
        min_stock_level -> Nullable<Integer>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StockLocationsLocationTypeEnum;

    stock_locations (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 9]
        location_type -> StockLocationsLocationTypeEnum,
        address -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::StockMovementsMovementTypeEnum;

    stock_movements (id) {
        id -> Integer,
        product_id -> Integer,
        location_from -> Nullable<Integer>,
        location_to -> Nullable<Integer>,
        quantity -> Integer,
        #[max_length = 10]
        movement_type -> StockMovementsMovementTypeEnum,
        #[max_length = 255]
        reference -> Nullable<Varchar>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    transaction_type (id) {
        id -> Integer,
        #[max_length = 100]
        name -> Varchar,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Integer,
        #[max_length = 255]
        name -> Varchar,
        #[max_length = 255]
        email -> Varchar,
        #[max_length = 255]
        password_hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::joinable!(inventory_adjustments -> products (product_id));
diesel::joinable!(inventory_adjustments -> stock_locations (location_id));
diesel::joinable!(journal -> ledger (ledger_id));
diesel::joinable!(journal -> transaction_type (transaction_type_id));
diesel::joinable!(ledger -> coa_master (coa_id));
diesel::joinable!(products -> product_categories (category_id));
diesel::joinable!(stock_movements -> products (product_id));

diesel::allow_tables_to_appear_in_same_query!(
    coa_master,
    exchange_rate,
    financial_year,
    inventory_adjustments,
    journal,
    ledger,
    product_categories,
    products,
    stock_locations,
    stock_movements,
    transaction_type,
    users,
);
