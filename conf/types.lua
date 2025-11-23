---@meta

--- @class Entry
--- @field id string
--- @field title string | nil
--- @field authors Person[]
--- @field contributors Person[]
--- @field summary string | nil
--- @field content Content | nil
--- @field source string | nil
--- @field categories Category[]
--- @field links Link[]
--- @field base string | nil
local Entry = {}

--- @param tag string
function Entry:has_category(tag)
end

--- @class Feed
--- @field id string
--- @field title string | nil
--- @field description string | nil
--- @field authors Person[]
--- @field contributors Person[]
--- @field links Link[]
--- @field categories Category[]
--- @field ttl number | nil
local Feed = {}

--- @param tag string
function Feed:has_category(tag)
end

--- @class Person
--- @field name string
--- @field uri string | nil
--- @field email string | nil

--- @class Category
--- @field term string
--- @field label string | nil
--- @field subcategories Category[]

--- @class Link
--- @field href string
--- @field rel string | nil
--- @field media_type string | nil
--- @field title string | nil

--- @class Content
--- @field type "body" | "link"
--- @field body string | nil
--- @field link Link | nil

--- @class AlertOpts
--- @field summary? string | nil
--- @field message? string | nil

--- Report to the user using a notification system
--- @param opts? AlertOpts
function alert(opts) end

--- Record the current event in a database
function record() end

--- Log the message
--- @param msg string
function log(msg) end

--- Run a command
--- @param cmd string
function exec(cmd) end
