use actix_web::{web, Error, HttpResponse};
use huiyu_db_util::huiyu_db_macros::datasource;
use huiyu_db_util::huiyu_db_mapper::query::base_mapper::BaseMapper;
use huiyu_db_util::huiyu_db_mapper_core::base::page::Page;
use huiyu_db_util::huiyu_db_mapper_core::query::query_wrapper::QueryWrapper;
use crate::common::result::Res;
use crate::entity::entities::{RoleEntity, UserEntity};
use crate::mapper::mappers::{RoleMapper, UserMapper};
use crate::param::param::{RoleQueryParam, UserQueryParam};

#[datasource("sqlite")]
pub(crate) async fn query_role_page(json: web::Json<RoleQueryParam>) ->Result<HttpResponse, Error>{
    let app_query_param = json.0;
    if app_query_param.current_page.is_none(){
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"参数不能为空")));
    }
    if app_query_param.page_size.is_none(){
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"参数不能为空")));
    }
    let mut query_wrapper = QueryWrapper::new();
    if app_query_param.id.is_some(){
        query_wrapper = query_wrapper.eq(RoleEntity::ID,app_query_param.id.unwrap());
    }
    if app_query_param.role_name.is_some(){
        query_wrapper = query_wrapper.eq(RoleEntity::ROLE_NAME,app_query_param.role_name.unwrap());
    }
    query_wrapper = query_wrapper.order_by_desc(RoleEntity::CREATE_TIME);
    let page_res = RoleMapper::select_page(Page::new(app_query_param.current_page.unwrap() as u64, app_query_param.page_size.unwrap() as u64), &query_wrapper).await;
    if page_res.is_err() {
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,page_res.err().unwrap().to_string().as_str())));
    }
    Ok(HttpResponse::Ok().json(Res::success(page_res.ok().unwrap())))
}
