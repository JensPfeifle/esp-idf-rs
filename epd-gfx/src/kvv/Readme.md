# KVV

## Haltestellen-IDs:

https://diva4-9.efa-bw.de/Documents/Kataster/Kataster.8212.html

Ettlingen Stadt 7001231
Hirtenweg 7000403


https://projekte.kvv-efa.de/sl3-alone/XSLT_DM_REQUEST?outputFormat=JSON&coordOutputFormat=WGS84[dd.ddddd]&depType=stopEvents&locationServe<rActive=1&mode=direct&name_dm=7000238&type_dm=stop&useOnlyStops=1&useRealtime=1&limit=10

curl 'https://www.efa-bw.de/nvbw/XSLT_DM_REQUEST?outputFormat=JSON&coordOutputFormat=WGS84%5Bdd.ddddd%5D&command=&itdLPxx_timeFormat=12&language=de&itdLPxx_useJs=1&std3_suggestMacro=std3_suggest&std3_commonMacro=dm&itdLPxx_contractor=&std3_contractorMacro=&includeCompleteStopSeq=1&mergeDep=1&useAllStops=1&name_dm=Ettlingen%252C+Ettlingen+Stadt&type_dm=any&itdDateDayMonthYear=20.05.2022&itdTime=16%253A59&itdDateTimeDepArr=dep&includedMeans=checkbox&useRealtime=1&std3_inclMOT_0Macro=true&std3_inclMOT_1Macro=true&std3_inclMOT_2Macro=true&std3_inclMOT_3Macro=true&std3_inclMOT_4Macro=true&std3_inclMOT_5Macro=true&std3_inclMOT_6Macro=true&std3_inclMOT_8Macro=true&std3_inclMOT_9Macro=true&std3_inclMOT_10Macro=true&std3_inclMOT_11Macro=true&inclMOT_7=1&inclMOT_12=1&inclMOT_13=1&inclMOT_14=1&inclMOT_15=1&inclMOT_16=1&inclMOT_17=1&inclMOT_18=1&inclMOT_19=1&imparedOptionsActive=1&nameInfo_dm=7001231&itdLPxx_multiStepDm=2&deleteAssignedStops=1&mode=direct&line=all&itdLPxx_snippet=1&inclMOT_0=true&itdLPxx_template=dmresults&_=1653058789339&'

curl 'https://www.efa-bw.de/nvbw/XSLT_DM_REQUEST?outputFormat=JSON&itdLPxx_timeFormat=24&language=de&mergeDep=1&useAllStops=1&name_dm=Ettlingen%252C+Ettlingen+Stadt&type_dm=any&itdDateDayMonthYear=20.05.2022&itdTime=16%253A59&itdDateTimeDepArr=dep&useRealtime=1&nameInfo_dm=7001231&deleteAssignedStops=1&mode=direct&line=all&limit=10' 

https://www.kvv.de/tunnelEfaDirect.php?action=XSLT_DM_REQUEST&outputFormat=JSON&language=de&name_dm=7001004&type_dm=stopID&useRealtime=1

curl 'https://www.efa-bw.de/nvbw/XSLT_TRIP_REQUEST2?sessionID=0&requestID=0&outputFormat=JSON&coordOutputFormat=WGS84%5Bdd.ddddd%5D&command=&itdLPxx_timeFormat=12&language=de&itdLPxx_useJs=1&std3_suggestMacro=std3_suggest&std3_commonMacro=trip&itdLPxx_contractor=&std3_contractorMacro=&name_origin=Stadt%252C+Ettlingen&type_origin=any&nameInfo_origin=7001231&name_destination=Hirtenweg%252C+Karlsruhe&type_destination=any&nameInfo_destination=7000403%253A%24Z1&itdDateDayMonthYear=20.05.2022&itdTime=16%253A20&itdTripDateTimeDepArr=dep&includedMeans=checkbox&useRealtime=1&std3_inclMOT_0Macro=true&std3_inclMOT_1Macro=true&std3_inclMOT_2Macro=true&std3_inclMOT_3Macro=true&std3_inclMOT_4Macro=true&std3_inclMOT_5Macro=true&std3_inclMOT_6Macro=true&std3_inclMOT_8Macro=true&std3_inclMOT_9Macro=true&std3_inclMOT_10Macro=true&std3_inclMOT_11Macro=true&inclMOT_7=1&inclMOT_12=1&inclMOT_13=1&inclMOT_14=1&inclMOT_15=1&inclMOT_16=1&inclMOT_17=1&inclMOT_18=1&inclMOT_19=1&lineRestriction=400&routeType=LEASTTIME&trITMOTvalue100=15&useProxFootSearch=on&levelPTMm=mainconnection&maxChanges=9&itdLPxx_cyclingActive=on&std3_inclMOT_107Macro=true&cycleSpeed=15&std3_routeTypeCycleMacro=fastest&elevFac=50&itdLPxx_walkingActive=on&std3_inclMOT_100Macro=true&changeSpeed=normal&imparedOptionsActive=1&name_via=&nameInfo_via=invalid&type_via=any&dwellTimeMinutes=&name_notVia=&nameInfo_notVia=invalid&type_notVia=stop&itdLPxx_snippet=1&itdLPxx_template=tripresults_pt_trip&computationType=sequence&_=1653056436121'
