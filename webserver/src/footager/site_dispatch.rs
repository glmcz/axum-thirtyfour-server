use fantoccini::{Client, error};
use fantoccini::error::CmdError;
use reqwest::Url;
use crate::webscrapping::domain::ChooseDomain;
use webscrapping::artgrid::run_artgrid_instance;
use crate::footager::user::FootageUserRequest;


// validate user url and return number corresponding to used Artgrid script:
// 0 artgrid
// 1 artlist
// 2 elements.envato.com
// 3 instead check for multile links in uuser_inputs ... TODO pack or bundle of artlist audio files check
async fn choose_url_site(client: &Client, user_url: &str) -> Result<String, error::CmdError> {
    match Url::parse(user_url) {
        Ok(url) => {
            let occur = url.as_str().matches("https://").count();
            if occur == 1
            {
                match ChooseDomain::from(url.host_str()) {
                    ChooseDomain::Artgrid =>  run_artgrid_instance(client, user_url).await,
                    ChooseDomain::Artlist =>  Ok("ChooseDomain::Artlist".to_owned()),
                    ChooseDomain::Envato  =>  Ok("ChooseDomain::Envato".to_owned()),
                    _ =>  Err(CmdError::WaitTimeout)
                }
            }
            else
            {
                //more then one link in user input string...
                Err(CmdError::WaitTimeout)
            }
        }
        _ =>  Err(CmdError::WaitTimeout)
    }
}
async fn open_new_tab(client: &Client) -> Result<(), error::CmdError>
{
        // not need to clone driver because we have it under Arc<AppState>. Each req clone driver anyway...
        // let handle = main_driver.window().await.unwrap();
        // if let Some(err) = main_driver.switch_to_window(handle).await.err(){
        //     println!("{:?}", err);
        // }

        if let Some(err) = client.new_window(true).await.err(){
            println!("{:?}", err);
            //TODO error
        }
       Ok(())
}

pub async fn run_site_instance(client: Option<&Client>, req: FootageUserRequest) -> Result<String, error::CmdError> {
    if let Some(client) = client
    {
        open_new_tab(client).await?;
        choose_url_site(client, req.url.as_str()).await
    }else { Err(CmdError::WaitTimeout) }
}