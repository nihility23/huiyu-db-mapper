use crate::common::result::Res;
use crate::entity::entities::UserEntity;
use crate::mapper::mappers::UserMapper;
use crate::param::UserQueryParam;
use actix_web::{web, Error, HttpResponse};
use huiyu_db_util::huiyu_db_macros::datasource;
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper_core::base::page::Page;
use huiyu_db_util::huiyu_db_mapper_core::query::query_wrapper::QueryWrapper;

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
        query_wrapper = query_wrapper.eq("id",app_query_param.id.unwrap().into());
    }
    if app_query_param.user_name.is_some(){
        query_wrapper = query_wrapper.eq("user_name",app_query_param.user_name.unwrap().into());
    }
    let page_res = UserMapper::select_page(Page::new(app_query_param.current_page.unwrap() as u64, app_query_param.page_size.unwrap() as u64), &query_wrapper).await;
    if page_res.is_err() {
        return Ok(HttpResponse::Ok().json(page_res.err().unwrap()));
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
    }else {
        UserMapper::update_by_key(&mut user_entity).await;
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