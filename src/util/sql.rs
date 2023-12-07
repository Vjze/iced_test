use mysql_async::{prelude::Queryable, Row};


use crate::gui::views::{BoxDataInfo, SnDataInfo};

use super::util::get_connect;

pub async fn get_box(s: String) -> Vec<BoxDataInfo> {
    let pool = get_connect("mes_Factory").await;
    // 从连接池中获取一个连接
    let mut conn = pool.get_conn().await.unwrap();
    // 查询代码
    let query_ty = format!("where Pack_no = '{}' ORDER BY CreateTime DESC", s);
    let testtype_none = "Pack_no,Sn,PN,WorkOrder,Creator,CreateTime";
    let box_code = format!(
        "select {0} from MaterialPackSn {1} ",
        testtype_none, query_ty
    );
    // 执行查询
    let results = conn
        .query_map(box_code, |(pno, sn, pn, workorder, creator, createtime)| {
            BoxDataInfo {
                pno,
                sn,
                pn,
                workorder,
                creator,
                createtime,
            }
        })
        .await
        .unwrap();
    drop(conn);
    pool.disconnect().await.unwrap();
    results
}
pub async fn get_sn(s: String) -> Vec<SnDataInfo> {
    let pool = get_connect("BOSAautotest_Data").await;

    // 从连接池中获取一个连接
    let mut conn = pool.get_conn().await.unwrap();
    // 查询代码
    let query_ty = format!("where SN = '{}'", s);
    let testtype_12 = "SN,ProductBill,TestType,Result,Ith,Po,Vf,Im,Rs,Sen,Res,ICC,Idark,Vbr,Xtalk,Kink_I,TestDate";
    let sn_code = format!(
        "select {} from MAC_10GBOSADATA  {}",
        testtype_12, query_ty
    );
    // 执行查询
    let results: Vec<Row> = conn
        .query(sn_code)
        .await
        .unwrap();
    drop(conn);
    pool.disconnect().await.unwrap();
    let mut v = vec![];
    for x in results.into_iter(){
        let sn = SnDataInfo{ 
            sn: String::from_utf8(Vec::try_from(x.clone().unwrap().get(0).unwrap().clone()).unwrap()).unwrap(), 
            product_bill: String::from_utf8(Vec::try_from(x.clone().unwrap().get(1).unwrap().clone()).unwrap()).unwrap(), 
            test_type: String::from_utf8(Vec::try_from(x.clone().unwrap().get(2).unwrap().clone()).unwrap()).unwrap(),
             result: String::from_utf8(Vec::try_from(x.clone().unwrap().get(3).unwrap().clone()).unwrap()).unwrap(), 
             ith: String::from_utf8(Vec::try_from(x.clone().unwrap().get(4).unwrap().clone()).unwrap()).unwrap(), 
             pf: String::from_utf8(Vec::try_from(x.clone().unwrap().get(5).unwrap().clone()).unwrap()).unwrap(), 
             vop: String::from_utf8(Vec::try_from(x.clone().unwrap().get(6).unwrap().clone()).unwrap()).unwrap(), 
             im: String::from_utf8(Vec::try_from(x.clone().unwrap().get(7).unwrap().clone()).unwrap()).unwrap(), 
             rs: String::from_utf8(Vec::try_from(x.clone().unwrap().get(8).unwrap().clone()).unwrap()).unwrap(), 
             sen: String::from_utf8(Vec::try_from(x.clone().unwrap().get(9).unwrap().clone()).unwrap()).unwrap(), 
             res: String::from_utf8(Vec::try_from(x.clone().unwrap().get(10).unwrap().clone()).unwrap()).unwrap(), 
             icc: String::from_utf8(Vec::try_from(x.clone().unwrap().get(11).unwrap().clone()).unwrap()).unwrap(), 
             idark: String::from_utf8(Vec::try_from(x.clone().unwrap().get(12).unwrap().clone()).unwrap()).unwrap(), 
             vbr: String::from_utf8(Vec::try_from(x.clone().unwrap().get(13).unwrap().clone()).unwrap()).unwrap(), 
             ixtalk: String::from_utf8(Vec::try_from(x.clone().unwrap().get(14).unwrap().clone()).unwrap()).unwrap(), 
             kink: String::from_utf8(Vec::try_from(x.clone().unwrap().get(15).unwrap().clone()).unwrap()).unwrap(), 
             testdate: String::from_utf8(Vec::try_from(x.clone().unwrap().get(16).unwrap().clone()).unwrap()).unwrap() };
            v.push(sn.clone())
    }
    v
}

pub async fn login_user(id: String, pa: String) -> bool {
    let i = id.clone();
    let pool = get_connect("login").await;
    // 从连接池中获取一个连接
    let mut conn = pool.get_conn().await.unwrap();
    // 执行查询
    let results: Vec<Row> = conn
        .query(format!("select name from login where name = '{}'", i))
        .await
        .unwrap();
    drop(conn);
    pool.disconnect().await.unwrap();

    if results.is_empty() {
        false
    } else {
        login_pass(id, pa).await
    }
}

pub async fn login_pass(id: String, p: String) -> bool {
    let i = id.clone();
    let pool = get_connect("login").await;
    // 从连接池中获取一个连接
    let mut conn = pool.get_conn().await.unwrap();
    // 执行查询
    let results: Vec<Row> = conn
        .query(format!("select password from login where name = '{}'", i))
        .await
        .unwrap();
    drop(conn);
    pool.disconnect().await.unwrap();
    let r = results.get(0).unwrap();
    let mut pass = String::new();
    for x in r.clone().unwrap() {
        let t = String::from_utf8(Vec::try_from(x).unwrap()).unwrap();
        pass = t
    }
    pass == p
}
