use crate::{models, Result, PresenceDb};
use warp::{http::StatusCode, reply, Reply};

pub async fn get_user_items(items_db: PresenceDb) -> Result<impl Reply> {
    let local_db = items_db.lock().await;
    let local_db: Vec<models::UserItem> = local_db.values().cloned().collect();
    Ok(reply::with_status(reply::json(&local_db), StatusCode::OK))
}

pub async fn create_user_item(
    mut user_item: models::UserItem,
    items_db: PresenceDb,
) -> Result<impl Reply> {
    println!("Received UserData: {:?}", user_item);
    let mut local_db = items_db.lock().await;
    let key_count = local_db.keys().len();
    user_item.id = Some(key_count).unwrap();
    local_db.insert(key_count, user_item.clone());

    Ok(reply::with_status(
        reply::json(&user_item),
        StatusCode::CREATED,
    ))
}

pub async fn get_user_item_by_id(id: usize, items_db: PresenceDb) -> Result<impl Reply> {
    let local_db = items_db.lock().await;
    let user_item: &models::UserItem = match local_db.get(&id) {
        Some(item) => item,
        _ => {
            return Ok(reply::with_status(
                reply::json(&"{}"),
                StatusCode::NOT_FOUND,
            ));
        }
    };
    Ok(reply::with_status(
        reply::json(&user_item),
        StatusCode::OK,
    ))
}

pub async fn update_user_item_by_id(
    id: usize,
    updated_data: models::UserItemUpdate,
    items_db: PresenceDb,
) -> Result<impl Reply> {
    let mut local_db = items_db.lock().await;
    let mut user_item = match local_db.get(&id) {
        Some(item) => item.clone(),
        _ => {
            return Ok(reply::with_status(
                reply::json(&"{}"),
                StatusCode::NOT_FOUND,
            ));
        }
    };

    match updated_data.name {
        Some(name) => {
            println!("updating name from {} to {}", user_item.name, name);
            user_item.name = name;
        }
        _ => {}
    };

    match updated_data.email {
        Some(email) => {
            println!(
                "updating email from {} to {}",
                user_item.email, email
            );
            user_item.email = email;
        }
        _ => {}
    };

    match updated_data.password {
        Some(password) => {
            println!(
                "updating password from {:?} to {:?}",
                user_item.password, password
            );
            user_item.password = password;
        }
        _ => {}
    };

    match updated_data.phone {
        Some(phone) => {
            println!(
                "updating phone from {} to {}",
                user_item.phone, phone
            );
            user_item.phone = phone;
        }
        _ => {}
    };

    match updated_data.status {
        Some(status) => {
            println!(
                "updating status from {} to {}",
                user_item.status, status
            );
            user_item.status = status;
        }
        _ => {}
    };

    *local_db.get_mut(&id).unwrap() = user_item.clone();

    Ok(reply::with_status(
        reply::json(&user_item),
        StatusCode::OK,
    ))
}

pub async fn delete_user_item_by_id(id: usize, items_db: PresenceDb) -> Result<impl Reply> {
    let mut local_db = items_db.lock().await;

    println!("deleting user item with id: {}", id);
    local_db.remove(&id);

    Ok(reply::with_status(
        reply::html("delete success"),
        StatusCode::OK,
    ))
}