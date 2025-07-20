# rusty_beginnings
dabbling with rust, learning the ropes and making tiny useful utilities

first rust tool: exif_etl
A small exif extract, transform, load tool to aggregate a folder worth of .jpeg/.jpg into a .csv, crafted to be as simple as possible. Simply compile the code, drop executable into the .jpeg folder and run, .csv will be spawned in the .jpeg folder. TOML file set to max optimization including cpu-native flag, will likely have to remove cpu-native if deploying executable across an enterprise. 
