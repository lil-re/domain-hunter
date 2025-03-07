use std::sync::MutexGuard;
use rusqlite::{Connection, Result};
use crate::database::extensions_api::create_extension;
use crate::database::connection::DB_CONNECTION;
use crate::models::Extension;

pub fn run_migrations() -> Result<()> {
    let conn = DB_CONNECTION.lock().expect("Failed to lock the database connection");

    create_wishlist_table(&conn)?;
    create_extension_table(&conn)?;
    create_default_extensions(&conn)?;

    Ok(())
}

/// Create the `wishlist` table
fn create_wishlist_table(conn: &MutexGuard<Connection>) -> Result<()> {
    let wishlist_result = conn.execute("
        CREATE TABLE IF NOT EXISTS wishlist (
            id INTEGER PRIMARY KEY,
            tld VARCHAR(20) NOT NULL,
            domain VARCHAR(70) NOT NULL,
            status VARCHAR(70) NOT NULL,
            selected TINYINT(1) NOT NULL
        )",
                                       [],
    );

    match wishlist_result {
        Ok(_) => {
            println!("MIGRATION => Successfully created 'wishlist' table.");
        }
        Err(error) => {
            println!("MIGRATION => Could not create 'wishlist' table.");
            println!("MIGRATION => {}", error);
        }
    }

    Ok(())
}

/// Create the `extension` table
fn create_extension_table(conn: &MutexGuard<Connection>) -> Result<()> {
    let extension_result = conn.execute("
        CREATE TABLE IF NOT EXISTS extension (
            id INTEGER PRIMARY KEY,
            tld VARCHAR(20) NOT NULL,
            name VARCHAR(70) NOT NULL,
            selected TINYINT(1) NOT NULL
        )",
                 [],
    );

    match extension_result {
        Ok(_) => {
            println!("MIGRATION => Successfully created 'extension' table.");
        }
        Err(error) => {
            println!("MIGRATION => Could not create 'extension' table.");
            println!("MIGRATION => {}", error);
        }
    }

    Ok(())
}

/// Create default domain name extensions table
fn create_default_extensions(conn: &MutexGuard<Connection>) -> Result<()> {
    let extensions = get_default_extensions();

    for extension in extensions.iter() {
        create_extension(&conn, &extension);
    }

    println!("MIGRATION => Successfully added default extensions.");

    Ok(())
}

/// Get the list of default domain name extensions
pub fn get_default_extensions() -> Vec<Extension> {
    // Transform content into a vector of Extension
    match serde_json::from_str(DEFAULT_EXTENSIONS) {
        Ok(result) => result,
        Err(error) => { panic!("{}", error) }
    }
}

pub const DEFAULT_EXTENSIONS: &str = r#"[
  {"tld":"com","name":"Commercial","selected":true},
  {"tld":"org","name":"Organization","selected":true},
  {"tld":"net","name":"Network","selected":true},
  {"tld":"info","name":"Information","selected":false},
  {"tld":"biz","name":"Business","selected":false},
  {"tld":"pro","name":"Professional","selected":false},
  {"tld":"name","name":"Personal Names","selected":false},
  {"tld":"mobi","name":"Mobile Devices","selected":false},
  {"tld":"ad","name":"Andorra","selected":false},
  {"tld":"ae","name":"United Arab Emirates","selected":false},
  {"tld":"af","name":"Afghanistan","selected":false},
  {"tld":"ag","name":"Antigua and Barbuda","selected":false},
  {"tld":"ai","name":"Anguilla","selected":false},
  {"tld":"al","name":"Albania","selected":false},
  {"tld":"am","name":"Armenia","selected":false},
  {"tld":"ao","name":"Angola","selected":false},
  {"tld":"ar","name":"Argentina","selected":false},
  {"tld":"as","name":"American Samoa","selected":false},
  {"tld":"at","name":"Austria","selected":false},
  {"tld":"au","name":"Australia","selected":false},
  {"tld":"aw","name":"Aruba","selected":false},
  {"tld":"ax","name":"Åland Islands","selected":false},
  {"tld":"az","name":"Azerbaijan","selected":false},
  {"tld":"ba","name":"Bosnia and Herzegovina","selected":false},
  {"tld":"bb","name":"Barbados","selected":false},
  {"tld":"bd","name":"Bangladesh","selected":false},
  {"tld":"be","name":"Belgium","selected":false},
  {"tld":"bf","name":"Burkina Faso","selected":false},
  {"tld":"bg","name":"Bulgaria","selected":false},
  {"tld":"bh","name":"Bahrain","selected":false},
  {"tld":"bi","name":"Burundi","selected":false},
  {"tld":"bj","name":"Benin","selected":false},
  {"tld":"bl","name":"Saint Barthélemy","selected":false},
  {"tld":"bm","name":"Bermuda","selected":false},
  {"tld":"bn","name":"Brunei","selected":false},
  {"tld":"bo","name":"Bolivia","selected":false},
  {"tld":"bq","name":"Bonaire, Sint Eustatius and Saba","selected":false},
  {"tld":"br","name":"Brazil","selected":false},
  {"tld":"bs","name":"Bahamas","selected":false},
  {"tld":"bt","name":"Bhutan","selected":false},
  {"tld":"bv","name":"Bouvet Island","selected":false},
  {"tld":"bw","name":"Botswana","selected":false},
  {"tld":"by","name":"Belarus","selected":false},
  {"tld":"bz","name":"Belize","selected":false},
  {"tld":"ca","name":"Canada","selected":false},
  {"tld":"cc","name":"Cocos (Keeling) Islands","selected":false},
  {"tld":"cd","name":"Democratic Republic of the Congo","selected":false},
  {"tld":"cf","name":"Central African Republic","selected":false},
  {"tld":"cg","name":"Republic of the Congo","selected":false},
  {"tld":"ch","name":"Switzerland","selected":false},
  {"tld":"ci","name":"Ivory Coast","selected":false},
  {"tld":"ck","name":"Cook Islands","selected":false},
  {"tld":"cl","name":"Chile","selected":false},
  {"tld":"cm","name":"Cameroon","selected":false},
  {"tld":"cn","name":"China","selected":false},
  {"tld":"co","name":"Colombia","selected":false},
  {"tld":"cr","name":"Costa Rica","selected":false},
  {"tld":"cu","name":"Cuba","selected":false},
  {"tld":"cv","name":"Cape Verde","selected":false},
  {"tld":"cw","name":"Curacao","selected":false},
  {"tld":"cx","name":"Christmas Island","selected":false},
  {"tld":"cy","name":"Cyprus","selected":false},
  {"tld":"cz","name":"Czech Republic","selected":false},
  {"tld":"de","name":"Germany","selected":false},
  {"tld":"dj","name":"Djibouti","selected":false},
  {"tld":"dk","name":"Denmark","selected":false},
  {"tld":"dm","name":"Dominica","selected":false},
  {"tld":"do","name":"Dominican Republic","selected":false},
  {"tld":"dz","name":"Algeria","selected":false},
  {"tld":"ec","name":"Ecuador","selected":false},
  {"tld":"ee","name":"Estonia","selected":false},
  {"tld":"eg","name":"Egypt","selected":false},
  {"tld":"eh","name":"Western Sahara","selected":false},
  {"tld":"er","name":"Eritrea","selected":false},
  {"tld":"es","name":"Spain","selected":false},
  {"tld":"et","name":"Ethiopia","selected":false},
  {"tld":"eu","name":"European Union","selected":false},
  {"tld":"fi","name":"Finland","selected":false},
  {"tld":"fj","name":"Fiji","selected":false},
  {"tld":"fm","name":"Micronesia","selected":false},
  {"tld":"fo","name":"Faroe Islands","selected":false},
  {"tld":"fr","name":"France","selected":false},
  {"tld":"ga","name":"Gabon","selected":false},
  {"tld":"gb","name":"United Kingdom","selected":false},
  {"tld":"gd","name":"Grenada","selected":false},
  {"tld":"ge","name":"Georgia","selected":false},
  {"tld":"gf","name":"French Guiana","selected":false},
  {"tld":"gg","name":"Guernsey","selected":false},
  {"tld":"gh","name":"Ghana","selected":false},
  {"tld":"gi","name":"Gibraltar","selected":false},
  {"tld":"gl","name":"Greenland","selected":false},
  {"tld":"gm","name":"Gambia","selected":false},
  {"tld":"gn","name":"Guinea","selected":false},
  {"tld":"go","name":"Greece","selected":false},
  {"tld":"gp","name":"Guadeloupe","selected":false},
  {"tld":"gq","name":"Equatorial Guinea","selected":false},
  {"tld":"gr","name":"Greece","selected":false},
  {"tld":"gs","name":"South Georgia and the South Sandwich Islands","selected":false},
  {"tld":"gt","name":"Guatemala","selected":false},
  {"tld":"gu","name":"Guam","selected":false},
  {"tld":"gw","name":"Guinea-Bissau","selected":false},
  {"tld":"gy","name":"Guyana","selected":false},
  {"tld":"hk","name":"Hong Kong","selected":false},
  {"tld":"hm","name":"Heard Island and McDonald Islands","selected":false},
  {"tld":"hn","name":"Honduras","selected":false},
  {"tld":"hr","name":"Croatia","selected":false},
  {"tld":"ht","name":"Haiti","selected":false},
  {"tld":"hu","name":"Hungary","selected":false},
  {"tld":"id","name":"Indonesia","selected":false},
  {"tld":"ie","name":"Ireland","selected":false},
  {"tld":"il","name":"Israel","selected":false},
  {"tld":"im","name":"Isle of Man","selected":false},
  {"tld":"in","name":"India","selected":false},
  {"tld":"io","name":"British Indian Ocean Territory","selected":false},
  {"tld":"iq","name":"Iraq","selected":false},
  {"tld":"ir","name":"Iran","selected":false},
  {"tld":"is","name":"Iceland","selected":false},
  {"tld":"it","name":"Italy","selected":false},
  {"tld":"je","name":"Jersey","selected":false},
  {"tld":"jm","name":"Jamaica","selected":false},
  {"tld":"jo","name":"Jordan","selected":false},
  {"tld":"jp","name":"Japan","selected":false},
  {"tld":"ke","name":"Kenya","selected":false},
  {"tld":"kg","name":"Kyrgyzstan","selected":false},
  {"tld":"kh","name":"Cambodia","selected":false},
  {"tld":"ki","name":"Kiribati","selected":false},
  {"tld":"km","name":"Comoros","selected":false},
  {"tld":"kn","name":"Saint Kitts and Nevis","selected":false},
  {"tld":"kp","name":"North Korea","selected":false},
  {"tld":"kr","name":"South Korea","selected":false},
  {"tld":"kw","name":"Kuwait","selected":false},
  {"tld":"ky","name":"Cayman Islands","selected":false},
  {"tld":"kz","name":"Kazakhstan","selected":false},
  {"tld":"la","name":"Laos","selected":false},
  {"tld":"lb","name":"Lebanon","selected":false},
  {"tld":"lc","name":"Saint Lucia","selected":false},
  {"tld":"li","name":"Liechtenstein","selected":false},
  {"tld":"lk","name":"Sri Lanka","selected":false},
  {"tld":"lr","name":"Liberia","selected":false},
  {"tld":"ls","name":"Lesotho","selected":false},
  {"tld":"lt","name":"Lithuania","selected":false},
  {"tld":"lu","name":"Luxembourg","selected":false},
  {"tld":"lv","name":"Latvia","selected":false},
  {"tld":"ly","name":"Libya","selected":false},
  {"tld":"ma","name":"Morocco","selected":false},
  {"tld":"mc","name":"Monaco","selected":false},
  {"tld":"md","name":"Moldova","selected":false},
  {"tld":"me","name":"Montenegro","selected":false},
  {"tld":"mf","name":"Saint Martin","selected":false},
  {"tld":"mg","name":"Madagascar","selected":false},
  {"tld":"mh","name":"Marshall Islands","selected":false},
  {"tld":"mk","name":"North Macedonia","selected":false},
  {"tld":"ml","name":"Mali","selected":false},
  {"tld":"mm","name":"Myanmar","selected":false},
  {"tld":"mn","name":"Mongolia","selected":false},
  {"tld":"mo","name":"Macau","selected":false},
  {"tld":"mp","name":"Northern Mariana Islands","selected":false},
  {"tld":"mq","name":"Martinique","selected":false},
  {"tld":"mr","name":"Mauritania","selected":false},
  {"tld":"ms","name":"Montserrat","selected":false},
  {"tld":"mt","name":"Malta","selected":false},
  {"tld":"mu","name":"Mauritius","selected":false},
  {"tld":"mv","name":"Maldives","selected":false},
  {"tld":"mw","name":"Malawi","selected":false},
  {"tld":"mx","name":"Mexico","selected":false},
  {"tld":"my","name":"Malaysia","selected":false},
  {"tld":"mz","name":"Mozambique","selected":false},
  {"tld":"na","name":"Namibia","selected":false},
  {"tld":"nc","name":"New Caledonia","selected":false},
  {"tld":"ne","name":"Niger","selected":false},
  {"tld":"nf","name":"Norfolk Island","selected":false},
  {"tld":"ng","name":"Nigeria","selected":false},
  {"tld":"ni","name":"Nicaragua","selected":false},
  {"tld":"nl","name":"Netherlands","selected":false},
  {"tld":"no","name":"Norway","selected":false},
  {"tld":"np","name":"Nepal","selected":false},
  {"tld":"nr","name":"Nauru","selected":false},
  {"tld":"nu","name":"Niue","selected":false},
  {"tld":"nz","name":"New Zealand","selected":false},
  {"tld":"om","name":"Oman","selected":false},
  {"tld":"pa","name":"Panama","selected":false},
  {"tld":"pe","name":"Peru","selected":false},
  {"tld":"pf","name":"French Polynesia","selected":false},
  {"tld":"pg","name":"Papua New Guinea","selected":false},
  {"tld":"ph","name":"Philippines","selected":false},
  {"tld":"pk","name":"Pakistan","selected":false},
  {"tld":"pl","name":"Poland","selected":false},
  {"tld":"pm","name":"Saint Pierre and Miquelon","selected":false},
  {"tld":"pn","name":"Pitcairn Islands","selected":false},
  {"tld":"pr","name":"Puerto Rico","selected":false},
  {"tld":"pt","name":"Portugal","selected":false},
  {"tld":"pw","name":"Palau","selected":false},
  {"tld":"py","name":"Paraguay","selected":false},
  {"tld":"qa","name":"Qatar","selected":false},
  {"tld":"re","name":"Réunion","selected":false},
  {"tld":"ro","name":"Romania","selected":false},
  {"tld":"rs","name":"Serbia","selected":false},
  {"tld":"ru","name":"Russia","selected":false},
  {"tld":"rw","name":"Rwanda","selected":false},
  {"tld":"sa","name":"Saudi Arabia","selected":false},
  {"tld":"sb","name":"Solomon Islands","selected":false},
  {"tld":"sc","name":"Seychelles","selected":false},
  {"tld":"sd","name":"Sudan","selected":false},
  {"tld":"se","name":"Sweden","selected":false},
  {"tld":"sg","name":"Singapore","selected":false},
  {"tld":"sh","name":"Saint Helena","selected":false},
  {"tld":"si","name":"Slovenia","selected":false},
  {"tld":"sj","name":"Svalbard and Jan Mayen","selected":false},
  {"tld":"sk","name":"Slovakia","selected":false},
  {"tld":"sl","name":"Sierra Leone","selected":false},
  {"tld":"sm","name":"San Marino","selected":false},
  {"tld":"sn","name":"Senegal","selected":false},
  {"tld":"so","name":"Somalia","selected":false},
  {"tld":"sr","name":"Suriname","selected":false},
  {"tld":"ss","name":"South Sudan","selected":false},
  {"tld":"st","name":"São Tomé and Príncipe","selected":false},
  {"tld":"sv","name":"El Salvador","selected":false},
  {"tld":"sx","name":"Sint Maarten","selected":false},
  {"tld":"sy","name":"Syria","selected":false},
  {"tld":"sz","name":"Eswatini","selected":false},
  {"tld":"tc","name":"Turks and Caicos Islands","selected":false},
  {"tld":"td","name":"Chad","selected":false},
  {"tld":"tf","name":"French Southern and Antarctic Lands","selected":false},
  {"tld":"tg","name":"Togo","selected":false},
  {"tld":"th","name":"Thailand","selected":false},
  {"tld":"tj","name":"Tajikistan","selected":false},
  {"tld":"tk","name":"Tokelau","selected":false},
  {"tld":"tl","name":"Timor-Leste","selected":false},
  {"tld":"tm","name":"Turkmenistan","selected":false},
  {"tld":"tn","name":"Tunisia","selected":false},
  {"tld":"to","name":"Tonga","selected":false},
  {"tld":"tr","name":"Turkey","selected":false},
  {"tld":"tt","name":"Trinidad and Tobago","selected":false},
  {"tld":"tv","name":"Tuvalu","selected":false},
  {"tld":"tz","name":"Tanzania","selected":false},
  {"tld":"ua","name":"Ukraine","selected":false},
  {"tld":"ug","name":"Uganda","selected":false},
  {"tld":"us","name":"United States","selected":false},
  {"tld":"uy","name":"Uruguay","selected":false},
  {"tld":"uz","name":"Uzbekistan","selected":false},
  {"tld":"va","name":"Vatican City","selected":false},
  {"tld":"vc","name":"Saint Vincent and the Grenadines","selected":false},
  {"tld":"ve","name":"Venezuela","selected":false},
  {"tld":"vg","name":"British Virgin Islands","selected":false},
  {"tld":"vi","name":"U.S. Virgin Islands","selected":false},
  {"tld":"vn","name":"Vietnam","selected":false},
  {"tld":"vu","name":"Vanuatu","selected":false},
  {"tld":"wf","name":"Wallis and Futuna","selected":false},
  {"tld":"ws","name":"Samoa","selected":false},
  {"tld":"ye","name":"Yemen","selected":false},
  {"tld":"yt","name":"Mayotte","selected":false},
  {"tld":"za","name":"South Africa","selected":false},
  {"tld":"zm","name":"Zambia","selected":false},
  {"tld":"zw","name":"Zimbabwe","selected":false}
]"#;
