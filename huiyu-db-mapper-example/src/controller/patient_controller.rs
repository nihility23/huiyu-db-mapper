use actix_web::{web, Error, HttpResponse};
use huiyu_db_mapper::huiyu_db_mapper_core::base::mapping::Mapping;
use huiyu_db_mapper::huiyu_db_mapper_core::base::page::Page;
use huiyu_db_mapper::huiyu_db_mapper_core::query::query_wrapper::QueryWrapper;
use huiyu_db_mapper::huiyu_db_mapper_impl::query::base_mapper::BaseMapper;
use huiyu_db_mapper::huiyu_db_mapper_macros::{datasource, transactional};
use crate::common::result::Res;
use crate::entity::entities::{EdPatientEntity, RoleEntity};
use crate::mapper::mappers::{EdPatientMapper, RoleMapper};
use crate::param::param::{PatientQueryParam, RoleQueryParam};

#[datasource("oracle11g")]
pub(crate) async fn query_patient_by_page(json: web::Json<PatientQueryParam>) ->Result<HttpResponse, Error>{
    let app_query_param = json.0;
    if app_query_param.current_page.is_none(){
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"参数不能为空")));
    }
    if app_query_param.page_size.is_none(){
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,"参数不能为空")));
    }
    let mut query_wrapper = QueryWrapper::new();
    if app_query_param.id.is_some(){
        query_wrapper = query_wrapper.eq(EdPatientEntity::ID,app_query_param.id.unwrap().to_string());
    }
    if app_query_param.patient_name.is_some(){
        query_wrapper = query_wrapper.eq(EdPatientEntity::PATIENT_NAME,app_query_param.patient_name.unwrap());
    }
    query_wrapper = query_wrapper.order_by_desc(EdPatientEntity::START_TIME);
    let page_res = EdPatientMapper::select_page(Page::new(app_query_param.current_page.unwrap() as u64, app_query_param.page_size.unwrap() as u64), &query_wrapper).await;
    if page_res.is_err() {
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1,page_res.err().unwrap().to_string().as_str())));
    }
    Ok(HttpResponse::Ok().json(Res::success(page_res.ok().unwrap())))
}

#[datasource("oracle11g")]
pub(crate) async fn insert_patient() ->Result<HttpResponse, Error>{
    let mut ed_patient = EdPatientEntity::new();
    ed_patient.id = Some(uuid::Uuid::new_v4().to_string());
    ed_patient.patient_id = Some(uuid::Uuid::new_v4().to_string());
    ed_patient.file_name = Some("file_name".to_string());
    ed_patient.site_id = Some("xdts".to_string());
    ed_patient.pad_id = Some("xdts".to_string());
    ed_patient.start_time = Some(chrono::Local::now());
    ed_patient.pc_file_state = Some("1".to_string());
    ed_patient.del_state = Some("2".to_string());
    ed_patient.pad_send_state = Some("2".to_string());
    ed_patient.pad_file_state = Some("2".to_string());

    ed_patient.report_state=Some("1".to_string());
    let res = transactional!({
        let insert_result = EdPatientMapper::insert(&mut ed_patient).await;
        insert_result
    });
    if res.is_err() {
        return Ok(HttpResponse::Ok().json(Res::<()>::fail(-1, res.err().unwrap().to_string().as_str())));
    }
    Ok(HttpResponse::Ok().json(Res::success(ed_patient.id.unwrap())))
}