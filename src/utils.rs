extern crate reqwest;

use ipnetwork::Ipv4Network;
use reqwest::header;

pub async fn get_all_ipv4() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let cidrs = get_cidr_from_cf().await?;
    let _cidrs_arr = cidrs.split("\n");
    let mut res = Vec::<String>::new();
    for cidr in _cidrs_arr {
        let temps: Ipv4Network = cidr.parse().unwrap();
        for temp in temps.iter() {
            res.push(String::from(temp.to_string()));
        }
    }
    return Ok(res);
}

pub async fn get_cidr_from_cf() -> Result<String, Box<dyn std::error::Error>> {
    let mut headers = header::HeaderMap::new();
    headers.insert("authority", "www.cloudflare.com".parse().unwrap());
    headers.insert("accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7".parse().unwrap());
    headers.insert("accept-language", "zh-CN,zh;q=0.9".parse().unwrap());
    headers.insert("cache-control", "no-cache".parse().unwrap());
    headers.insert(header::COOKIE, "sparrow_id=%7B%22deviceId%22%3A%22ea7e320c-91dc-4383-883b-47b3c35ab8c9%22%2C%22userId%22%3A%2299f0645146f0f6425892970e65619f72%22%7D; facebook-pixel_dzQR__fb-pixel=fb.2.1705404000367.1513710372; google-analytics_v4_nzcr__ga4=624be9f8-4321-458d-9db8-c9a89050799d; google-analytics_v4_60a4__ga4=e5205435-ff71-4c8f-9b49-c706bbc6a752; facebook-pixel_OwdI__fb-pixel=fb.2.1706331823483.604593788; google-analytics_v4_SQqy__ga4=16225c37-9a1a-46ca-b6fd-ee0d9263cbe0; google-analytics_v4_SQqy___z_ga_audiences=16225c37-9a1a-46ca-b6fd-ee0d9263cbe0; facebook-pixel_VVgx__fb-pixel=fb.2.1706331823483.98846858; facebook-pixel_elKW__fb-pixel=fb.2.1706331823483.570433449; cfmrk_cic={\"id\":\"7VsTLFkR+Cw6rbps6udC4z8viksFS1CV\",\"v1\":0,\"v2\":0,\"v3\":0,\"v5\":0,\"v7\":0,\"v8\":0,\"v6\":0}; cf_clearance=Wo.yRhY8Gsn880y5L9TZxWNDeNiPoTAXdcSPJpGOpno-1706852312-1-AcGY17/Qu3rXMgXGgvLzllAyRYh/4SSBpI13kQQc1K//wzpUn4lqVkzIo4MMiqwwSEmuj/6l8N9mvaxYjzGnIKM=; _gcl_au=1.1.2077158720.1706852316; _gd_visitor=6df0a792-2d7f-4b6f-8fd1-2764834f8f4d; _gd_svisitor=af267368c65701001490a2653b0100004d3fdd00; _biz_uid=ea97fbb7077541e7e9a8a717ceacfbd2; _mkto_trk=id:713-XSC-918&token:_mch-cloudflare.com-1706861973308-90009; _biz_flagsA=%7B%22Version%22%3A1%2C%22XDomain%22%3A%221%22%2C%22ViewThrough%22%3A%221%22%2C%22Mkto%22%3A%221%22%7D; google-analytics_v4_nzcr__engagementDuration=0; google-analytics_v4_nzcr__session_counter=4; __cf_logged_in=1; CF_VERIFIED_DEVICE_50743f6c08bb150d88ee31f2e024a25b57bfb794fffe1d9db6406e59ca6fe1be=1706969413; google-analytics_v4_nzcr__engagementStart=1706969438287; google-analytics_v4_nzcr__counter=51; google-analytics_v4_nzcr__let=1706969438287; _ga_8BK794H3J9=GS1.1.1707132911.3.0.1707132912.0.0.0; __cf_bm=UjasGuOYEPLASkXo3kufv0pcwxXhwbq8qiHlEqrmopI-1707211571-1-AeJk6CHRQ3yhRalBJWsX/7UKndYSkzo+7X+j7kyenXCD7Yjknj/EzX66ekhMz4DtJwsu2toudHG+/Ai8c45VezhKKB2/8YcjgQXdwEzJdbQg; at_check=true; google-analytics_v4_60a4__ga4sid=709443026; google-analytics_v4_60a4__session_counter=7; google-analytics_v4_SQqy__ga4sid=676288768; google-analytics_v4_SQqy__session_counter=7; mboxEdgeCluster=34; _gid=GA1.2.691401436.1707211577; _gat_UA-10218544-29=1; OptanonConsent=isGpcEnabled=0&datestamp=Tue+Feb+06+2024+17%3A26%3A21+GMT%2B0800+(%E4%B8%AD%E5%9B%BD%E6%A0%87%E5%87%86%E6%97%B6%E9%97%B4)&version=202308.2.0&browserGpcFlag=0&isIABGlobal=false&hosts=&consentId=5eb85da8-87c0-4add-b238-225968b6bdd6&interactionCount=1&landingPath=NotLandingPage&groups=C0001%3A1%2CC0003%3A1%2CC0002%3A1%2CC0004%3A1%2CSSPD_BG%3A1&AwaitingReconsent=false; google-analytics_v4_60a4__counter=9; google-analytics_v4_60a4__let=1707211581690; google-analytics_v4_SQqy__counter=9; google-analytics_v4_SQqy__let=1707211581690; reddit_fZaD__reddit_uuid=1707211581690.a7cf9327-96d4-4cda-bae4-634f4576731f; _gd_session=f28b9ce0-8c4d-455a-89c6-576231ef504d; _ga=GA1.1.1834914993.1706852316; mbox=PC#f3b11f17e78c466bb9c413239448273b.34_0#1770456383|session#bddab16241054289b4555c2118bd914c#1707213443; _biz_nA=6; _biz_pendingA=%5B%5D; drift_campaign_refresh=e052d270-6fdc-4c9c-ba48-2f8fce2e7503; _ga_PHVG60J2FD=GS1.1.1707211577.3.1.1707211584.0.0.0; _ga_SQCRB0TXZW=GS1.1.1707211576.4.1.1707211584.0.0.0; google-analytics_v4_60a4__engagementDuration=3332; google-analytics_v4_60a4__engagementStart=1707211585022; google-analytics_v4_SQqy__engagementDuration=3332; google-analytics_v4_SQqy__engagementStart=1707211585022".parse().unwrap());
    headers.insert("pragma", "no-cache".parse().unwrap());
    headers.insert(
        "referer",
        "https://www.cloudflare.com/zh-cn/ips/".parse().unwrap(),
    );
    headers.insert(
        "sec-ch-ua",
        "\"Not A(Brand\";v=\"99\", \"Google Chrome\";v=\"121\", \"Chromium\";v=\"121\""
            .parse()
            .unwrap(),
    );
    headers.insert("sec-ch-ua-arch", "\"arm\"".parse().unwrap());
    headers.insert("sec-ch-ua-bitness", "\"64\"".parse().unwrap());
    headers.insert(
        "sec-ch-ua-full-version",
        "\"121.0.6167.139\"".parse().unwrap(),
    );
    headers.insert("sec-ch-ua-full-version-list", "\"Not A(Brand\";v=\"99.0.0.0\", \"Google Chrome\";v=\"121.0.6167.139\", \"Chromium\";v=\"121.0.6167.139\"".parse().unwrap());
    headers.insert("sec-ch-ua-mobile", "?0".parse().unwrap());
    headers.insert("sec-ch-ua-model", "\"\"".parse().unwrap());
    headers.insert("sec-ch-ua-platform", "\"macOS\"".parse().unwrap());
    headers.insert("sec-ch-ua-platform-version", "\"14.3.0\"".parse().unwrap());
    headers.insert("sec-fetch-dest", "document".parse().unwrap());
    headers.insert("sec-fetch-mode", "navigate".parse().unwrap());
    headers.insert("sec-fetch-site", "same-origin".parse().unwrap());
    headers.insert("sec-fetch-user", "?1".parse().unwrap());
    headers.insert("upgrade-insecure-requests", "1".parse().unwrap());
    headers.insert("user-agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36".parse().unwrap());

    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();
    let res = client
        .get("https://www.cloudflare.com/ips-v4/")
        .headers(headers)
        .send()
        .await?;

    Ok(res.text().await.unwrap())
}
