extern crate chrono;

use chrono::prelude::*;
use chrono::{DateTime};

// Vi har att skapa en ny trait första.
trait DateTimeExt 
{
    fn veckan(&self) -> u32;
}

// Med den ny trait, vi göra att skapa en ny impl.
impl DateTimeExt for DateTime<chrono::offset::Utc> 
{
    fn veckan(&self) -> u32 
    {
        let now = self.date().iso_week().week();
	return now;
    }
}

fn main() 
{
    // Och här vi har att omfatta våra ny "type".
    // Du ska att göra detsamma när du vill att omfatta den ny "type".
    use DateTimeExt;

    let dt = Utc::now();
    println!("{}", dt.veckan());
}
