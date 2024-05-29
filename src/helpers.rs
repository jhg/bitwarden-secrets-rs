use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum BwObject {
    Item,
    Username,
    Password,
    Uri,
    Totp,
    Notes,
    Exposed,
    Attachment,
    Folder,
    Collection,
    OrgCollection,
    Organization,
    Template,
    Fingerprint,
    Send,
}

impl BwObject {
    pub fn as_str(&self) -> &str {
        match self {
            BwObject::Item => "item",
            BwObject::Username => "username",
            BwObject::Password => "password",
            BwObject::Uri => "uri",
            BwObject::Totp => "totp",
            BwObject::Notes => "notes",
            BwObject::Exposed => "exposed",
            BwObject::Attachment => "attachment",
            BwObject::Folder => "folder",
            BwObject::Collection => "collection",
            BwObject::OrgCollection => "orgcollection",
            BwObject::Organization => "organization",
            BwObject::Template => "template",
            BwObject::Fingerprint => "fingerprint",
            BwObject::Send => "send",
        }
    }

    pub fn is_a_secret(&self) -> bool {
        match self {
            BwObject::Username
            | BwObject::Password
            | BwObject::Uri
            | BwObject::Totp
            | BwObject::Notes
            => true,
            _ => false,
        }
    }
}
