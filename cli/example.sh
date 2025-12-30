#!/bin/bash
#df_dir="/Users/nwesterhausen/Library/Containers/com.isaacmarovitz.Whisky/Bottles/CCC4A738-378C-472A-A1A5-F2F4259D1FD9/drive_c/Program Files (x86)/Steam/steamapps/common/Dwarf Fortress"
# df_dir="/C/Program Files (x86)/Steam/steamapps/common/Dwarf Fortress"
df_dir="/Volumes/LaCie/Samples/Dwarf_Fortress"
user_data_dir="/Volumes/LaCie/Samples/Bay_12_Games/Dwarf Fortress"
# cargo run -- --vanilla -o vanilla.json -P -U "$user_data_dir" "$df_dir"
cargo run -- --installed -o installed-mods.json -P -U "$user_data_dir" "$df_dir"
