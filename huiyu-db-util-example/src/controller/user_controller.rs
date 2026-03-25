use crate::common::result::Res;
use crate::entity::entities::UserEntity;
use crate::mapper::mappers::UserMapper;
use actix_web::{web, Error, HttpResponse};
use tracing::error;
use huiyu_db_util::huiyu_db_macros::datasource;
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper_core::base::page::{Page, PageRes};
use huiyu_db_util::huiyu_db_mapper_core::query::query_wrapper::QueryWrapper;
use crate::entity::mappings::RoleDTO;
use crate::param::param::UserQueryParam;

#[datasource("sqlite")]
pub(crate) async fn query_user_page(json: web::Json<UserQueryParam>) ->Result<HttpResponse, Error>{
    let app_query_param = json.0;
    if app_query_param.current_page.is_none(){
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"参数不能为空")));
    }
    if app_query_param.page_size.is_none(){
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"参数不能为空")));
    }
    let mut query_wrapper = QueryWrapper::new();
    if app_query_param.id.is_some(){
        query_wrapper = query_wrapper.eq(UserEntity::ID,app_query_param.id.unwrap());
    }
    if app_query_param.user_name.is_some(){
        query_wrapper = query_wrapper.eq(UserEntity::USERNAME,app_query_param.user_name.unwrap());
    }
    query_wrapper = query_wrapper.order_by_desc(UserEntity::CREATE_TIME);
    let page_res = UserMapper::select_page(Page::new(app_query_param.current_page.unwrap() as u64, app_query_param.page_size.unwrap() as u64), &query_wrapper).await;
    if page_res.is_err() {
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,page_res.err().unwrap().to_string().as_str())));
    }
    Ok(HttpResponse::Ok().json(Res::success(page_res.ok().unwrap())))
}

pub(crate) async fn delete_user(id: web::Path<i64>) ->Result<HttpResponse, Error>{
    let app_entity_res = UserMapper::select_by_key(&id.into_inner()).await;
    if app_entity_res.is_err() {
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"文件未找到")));
    }
    let app_entity_opt = app_entity_res.unwrap();
    if app_entity_opt.is_none() {
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"文件未找到")));
    }
    let app_entity = app_entity_opt.unwrap();
    let res = UserMapper::delete_by_key(&app_entity.id.unwrap()).await;
    if res.is_err() {
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,res.unwrap_err().to_string().as_str())));
    }
    Ok(HttpResponse::Ok().json(Res::<()>::success_without_res()))
}

#[datasource("sqlite")]
pub(crate) async fn save_user(user_entity_json: web::Json<UserEntity>) ->Result<HttpResponse, Error>{
    let mut user_entity = user_entity_json.0;
    if user_entity.id.is_none(){
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"参数不能为空")));
    }
    let user_entity_res = UserMapper::select_by_key(&user_entity.id.unwrap()).await;
    if user_entity_res.is_err(){
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,user_entity_res.unwrap_err().to_string().as_str())));
    }
    let user_entity_in_db = user_entity_res.unwrap();
    if user_entity_in_db.is_none(){
        // 插入
        let res = UserMapper::insert(&mut user_entity).await;
        if res.is_err() {
            return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,res.unwrap_err().to_string().as_str())));
        }
    }else {
        UserMapper::update_by_key(&user_entity).await.expect("TODO: panic message");
    }

    Ok(HttpResponse::Ok().json(Res::<()>::success_without_res()))
}

pub(crate) async fn query_user_by_id(id: web::Path<i64>) ->Result<HttpResponse, Error>{
    let user_res = UserMapper::select_by_key(&id.into_inner()).await;
    if user_res.is_err(){
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"user record not found".to_string().as_str())));
    }
    let user_entity_opt = user_res.unwrap();
    if user_entity_opt.is_none() {
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"user record not found".to_string().as_str())));
    }
    let mut user_entity = user_entity_opt.unwrap();
    user_entity.password = None;
    Ok(HttpResponse::Ok().json(Res::<UserEntity>::success(user_entity)))
}

#[datasource("sqlite")]
pub(crate) async fn query_user_name_by_id(id: web::Path<i64>) ->Result<HttpResponse, Error>{
    let user_name_res = UserMapper::select_name_by_id(id.into_inner()).await;
    if user_name_res.is_err(){
        error!("{}", user_name_res.err().unwrap().to_string());
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"user name not found".to_string().as_str())));
    }
    let user_name_opt = user_name_res.unwrap();
    if user_name_opt.is_none(){
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"user name is null".to_string().as_str())));
    }
    Ok(HttpResponse::Ok().json(Res::<String>::success(user_name_opt.unwrap())))
}

#[datasource("sqlite")]
pub(crate) async fn query_user_name_by_page(json: web::Json<UserQueryParam>) ->Result<HttpResponse, Error>{
    let user_name_res = UserMapper::select_name_by_page(Page::new(json.0.current_page.unwrap() as u64, json.0.page_size.unwrap() as u64), json.0.user_name.unwrap()).await;
    if user_name_res.is_err(){
        error!("{}", user_name_res.err().unwrap().to_string()); 
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"user name not found".to_string().as_str())));
    }

    Ok(HttpResponse::Ok().json(Res::success(user_name_res.ok().unwrap())))
}