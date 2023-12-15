use snowflake_connector_rs::{SnowflakeRow, SnowflakeSession};

#[derive(Clone)]
pub struct CapacityRepository {
    pub session: &'static SnowflakeSession,
}

pub enum CapacityError {
    NotFound,
    Other,
}

pub struct Error {
    pub message: String,
    pub error: CapacityError,
}

impl CapacityRepository {
    pub async fn get_capacity(&self) -> Result<Vec<crate::models::capacity::Capacity>, Error> {
        let query = "SELECT * FROM STG_AMEX.VIC.CAPACITY_DATA";
        let rows = self.session.query(query).await;

        if let Err(err) = rows {
            return Err(Error {
                message: format!("Error querying capacity: {}", err),
                error: CapacityError::Other,
            });
        }

        let rows = rows.unwrap();

        let mut capacities: Vec<crate::models::capacity::Capacity> = Vec::new();
        for row in rows {
            let capacity = construct_capacity_from_row(&row).expect("Error constructing capacity");
            capacities.push(capacity);
        }

        Ok(capacities)
    }

    pub async fn get_capacity_by_id(
        &self,
        id: i64,
    ) -> Result<crate::models::capacity::Capacity, Error> {
        let query = format!("SELECT * FROM STG_AMEX.VIC.CAPACITY_DATA WHERE ID = {}", id);
        let rows = self.session.query(query).await;

        if let Err(err) = rows {
            return Err(Error {
                message: format!("Error querying capacity with id {}: {}", id, err),
                error: CapacityError::Other,
            });
        }

        let rows = rows.unwrap();
        if rows.len() == 0 {
            return Err(Error {
                message: format!("Capacity with id {} not found", id),
                error: CapacityError::NotFound,
            });
        }

        let mut capacities: Vec<crate::models::capacity::Capacity> = Vec::new();

        for row in rows {
            let capacity = construct_capacity_from_row(&row).expect("Error constructing capacity");
            capacities.push(capacity);
        }

        Ok(capacities[0].clone())
    }

    pub async fn create_capacity(
        &self,
        new_capacity: crate::models::capacity::NewCapacity,
    ) -> Result<(), Error> {
        let query = format!(
            "INSERT INTO STG_AMEX.VIC.CAPACITY_DATA (NAME, EMAIL, CITY_LOCATION, START_TIME, COMPLETION_TIME, OPERATION_MANAGER, VEHICLE_TYPE_NEEDED, NUMBER_OF_ICS_NEEDED, TSA_REQUIRED, MARKEN, PARCEL_SHIELD, DG, DG7, MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY) VALUES ('{}', '{}', '{}', '{}', '{}', '{}', '{}', {}, '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}', '{}')",
            new_capacity.name,
            new_capacity.email,
            new_capacity.city_location,
            new_capacity.start_time,
            new_capacity.completion_time,
            new_capacity.operation_manager,
            new_capacity.vehicle_type_needed,
            new_capacity.number_of_ics_needed,
            new_capacity.tsa_required.unwrap_or("".to_string()),
            new_capacity.marken.unwrap_or("".to_string()),
            new_capacity.parcel_shield.unwrap_or("".to_string()),
            new_capacity.dg.unwrap_or("".to_string()),
            new_capacity.dg7.unwrap_or("".to_string()),
            new_capacity.monday.unwrap_or("".to_string()),
            new_capacity.tuesday.unwrap_or("".to_string()),
            new_capacity.wednesday.unwrap_or("".to_string()),
            new_capacity.thursday.unwrap_or("".to_string()),
            new_capacity.friday.unwrap_or("".to_string()),
            new_capacity.saturday.unwrap_or("".to_string()),
            new_capacity.sunday.unwrap_or("".to_string()),
        );

        let res = self.session.query(query).await;

        if let Err(err) = res {
            return Err(Error {
                message: format!("Error creating capacity: {}", err),
                error: CapacityError::Other,
            });
        }

        Ok(())
    }

    pub async fn update_capacity(
        &self,
        id: i64,
        new_capacity: crate::models::capacity::NewCapacity,
    ) -> Result<(), Error> {
        let query = format!(
            "UPDATE STG_AMEX.VIC.CAPACITY_DATA SET NAME = '{}', EMAIL = '{}', CITY_LOCATION = '{}', START_TIME = '{}', COMPLETION_TIME = '{}', OPERATION_MANAGER = '{}', VEHICLE_TYPE_NEEDED = '{}', NUMBER_OF_ICS_NEEDED = {}, TSA_REQUIRED = '{}', MARKEN = '{}', PARCEL_SHIELD = '{}', DG = '{}', DG7 = '{}', MONDAY = '{}', TUESDAY = '{}', WEDNESDAY = '{}', THURSDAY = '{}', FRIDAY = '{}', SATURDAY = '{}', SUNDAY = '{}' WHERE ID = {}",
            new_capacity.name,
            new_capacity.email,
            new_capacity.city_location,
            new_capacity.start_time,
            new_capacity.completion_time,
            new_capacity.operation_manager,
            new_capacity.vehicle_type_needed,
            new_capacity.number_of_ics_needed,
            new_capacity.tsa_required.unwrap_or("".to_string()),
            new_capacity.marken.unwrap_or("".to_string()),
            new_capacity.parcel_shield.unwrap_or("".to_string()),
            new_capacity.dg.unwrap_or("".to_string()),
            new_capacity.dg7.unwrap_or("".to_string()),
            new_capacity.monday.unwrap_or("".to_string()),
            new_capacity.tuesday.unwrap_or("".to_string()),
            new_capacity.wednesday.unwrap_or("".to_string()),
            new_capacity.thursday.unwrap_or("".to_string()),
            new_capacity.friday.unwrap_or("".to_string()),
            new_capacity.saturday.unwrap_or("".to_string()),
            new_capacity.sunday.unwrap_or("".to_string()),
            id
        );

        let res = self.session.query(query).await;

        if let Err(err) = res {
            return Err(Error {
                message: format!("Error updating capacity: {}", err),
                error: CapacityError::Other,
            });
        }

        for row in res.unwrap() {
            let number_of_rows_updated = row.get::<i64>("NUMBER OF ROWS UPDATED").unwrap();
            if number_of_rows_updated == 0 {
                return Err(Error {
                    message: format!("Capacity with id {} not found", id),
                    error: CapacityError::NotFound,
                });
            }
        }

        Ok(())
    }

    pub async fn delete_capacity(&self, id: i64) -> Result<(), Error> {
        let query = format!("DELETE FROM STG_AMEX.VIC.CAPACITY_DATA WHERE ID = {}", id);

        let res = self.session.query(query).await;
        if let Err(err) = res {
            return Err(Error {
                message: format!("Error deleting capacity: {}", err),
                error: CapacityError::Other,
            });
        }

        for row in res.unwrap() {
            let number_of_rows_deleted = row.get::<i64>("NUMBER OF ROWS DELETED").unwrap();
            if number_of_rows_deleted == 0 {
                return Err(Error {
                    message: format!("Capacity with id {} not found", id),
                    error: CapacityError::NotFound,
                });
            }
        }

        Ok(())
    }
}

fn construct_capacity_from_row(
    row: &SnowflakeRow,
) -> Result<crate::models::capacity::Capacity, snowflake_connector_rs::Error> {
    let capacity = crate::models::capacity::Capacity {
        id: row.get::<i64>("ID")?,
        city_location: row.get::<String>("CITY_LOCATION")?,
        start_time: row.get::<String>("START_TIME")?,
        completion_time: row.get::<String>("COMPLETION_TIME")?,
        last_modified_time: row.get::<Option<String>>("LAST_MODIFIED_TIME")?,
        email: row.get::<String>("EMAIL")?,
        name: row.get::<String>("NAME")?,
        operation_manager: row.get::<String>("OPERATION_MANAGER")?,
        vehicle_type_needed: row.get::<String>("VEHICLE_TYPE_NEEDED")?,
        number_of_ics_needed: row.get::<i32>("NUMBER_OF_ICS_NEEDED")?,
        tsa_required: row.get::<Option<String>>("TSA_REQUIRED")?,
        marken: row.get::<Option<String>>("MARKEN")?,
        parcel_shield: row.get::<Option<String>>("PARCEL_SHIELD")?,
        dg: row.get::<Option<String>>("DG")?,
        dg7: row.get::<Option<String>>("DG7")?,
        monday: row.get::<Option<String>>("MONDAY")?,
        tuesday: row.get::<Option<String>>("TUESDAY")?,
        wednesday: row.get::<Option<String>>("WEDNESDAY")?,
        thursday: row.get::<Option<String>>("THURSDAY")?,
        friday: row.get::<Option<String>>("FRIDAY")?,
        saturday: row.get::<Option<String>>("SATURDAY")?,
        sunday: row.get::<Option<String>>("SUNDAY")?,
    };

    Ok(capacity)
}
