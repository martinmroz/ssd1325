var searchIndex = {};
searchIndex["ssd1325"] = {"doc":"","items":[[3,"Ssd1325","ssd1325","An SSD1325 display interface command adapter.",null,null],[4,"DisplayError","","Errors which may occur interacting with the display.",null,null],[13,"WriteFailed","","It was not possible to send all the necessary data to the display.",0,null],[4,"DisplayMode","","Mode of the primary communication channel.",null,null],[13,"Idle","","Interface idle. Not requested directly, but should be used when idle.",1,null],[13,"Reset","","Display reset mode.",1,null],[13,"Data","","Display idle mode. Cannot be in Reset, but can be either Data or Command.",1,null],[13,"Command","","Display command transport mode.",1,null],[8,"ControlChannel","","Responsible for placing the display in a given mode prior to executing a command.",null,null],[10,"run_in_mode","","Put the display communication channel in the specified `mode`. Once the command is executed the display must be left in a state other than `Reset`.",2,{"inputs":[{"name":"self"},{"name":"displaymode"},{"name":"fnmut"}],"output":{"name":"result"}}],[11,"clone","","",0,{"inputs":[{"name":"self"}],"output":{"name":"displayerror"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",0,{"inputs":[{"name":"self"},{"name":"displayerror"}],"output":{"name":"bool"}}],[11,"fmt","","",0,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"description","","",0,{"inputs":[{"name":"self"}],"output":{"name":"str"}}],[11,"clone","","",1,{"inputs":[{"name":"self"}],"output":{"name":"displaymode"}}],[11,"fmt","","",1,{"inputs":[{"name":"self"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"self"},{"name":"displaymode"}],"output":{"name":"bool"}}],[11,"new","","Returns a new instance of the receiver. The `transport` instance is used to send data to the display, typically over SPI although the MCU interface can be used if a suitable adapter is provided. The `control_channel` is used to put the display into a given mode before writing data. Typically, this is done using sysfs gpio. The display must be initialized prior to use, and is left Off.",3,{"inputs":[{"name":"write"},{"name":"controlchannel"}],"output":{"name":"self"}}],[11,"init","","Resets and initializes the display. Blocks for approximately 600ms.",3,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[11,"clear","","Clears the display.",3,{"inputs":[{"name":"self"}],"output":{"name":"result"}}],[11,"set_on","","Turn the display on or off. Configured to Off after initialization.",3,{"inputs":[{"name":"self"},{"name":"bool"}],"output":{"name":"result"}}],[11,"set_inverted","","Make the display inverted or normal. Configured to Normal after initialization.",3,{"inputs":[{"name":"self"},{"name":"bool"}],"output":{"name":"result"}}],[11,"blit_l1","","Send an entire bitmap frame to the display. The input image must be a 1-bit bitmap image arranged as 64 rows of 128 pixels. Pixels must be packed 8 per byte, with the most significant bit corresponding to the first pixel in the group (i.e. `0b1234567`).",3,null]],"paths":[[4,"DisplayError"],[4,"DisplayMode"],[8,"ControlChannel"],[3,"Ssd1325"]]};
initSearch(searchIndex);
