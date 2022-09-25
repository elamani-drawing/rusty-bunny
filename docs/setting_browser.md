# Summary
-[Firefox](#setting-firefox)  
-[Chrome](#setting-chrome)

## Setting Firefox
To set up the new application to be used as a custom search engine in Firefox, you can follow these steps:

1. Install the “Add custom search engine” Firefox Add-on
2. Open up the extension
3. Fill out the form with the following values:
```sh
    - Name: Rusty Bunny Local (you can use whatever you want here)
    - Search URL: http://localhost:8000/search?cmd=%s
```
4. Click “Add custom search engine”
5. Check the box “Make this the current search engine”
6. Click “Add”


## Setting Chrome
To set up the new application to be used as a custom search engine in Firefox, you can follow these steps:

1. Navigate to chrome://settings/searchEngines
2. Click “Add” under “Default Search Engines” and use the following values:
```sh
    - Search Engine: Rusty Bunny Local
    - Keyword: rb (triggers the search engine, if this search engine is not the default)
    - URL: http://localhost:8000/search?cmd=%s
```
3. Under “Other search engines”, find your search engine, select the 3 dots menu and select “Make default”