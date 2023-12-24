pub const EXAMPLE1: &str = "\
1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9";

pub const INPUT: &str = "\
2,3,318~2,3,320
6,4,233~6,5,233
7,3,165~7,5,165
8,8,289~8,9,289
0,1,5~0,2,5
9,3,3~9,3,3
0,1,136~0,3,136
9,2,245~9,2,247
4,5,35~7,5,35
2,0,164~2,1,164
4,3,137~8,3,137
0,1,332~0,1,333
5,4,183~6,4,183
9,0,121~9,2,121
2,1,131~3,1,131
8,1,304~8,3,304
4,3,186~4,4,186
2,8,105~5,8,105
5,4,167~7,4,167
4,7,12~4,9,12
5,4,275~5,6,275
0,4,220~0,5,220
2,3,205~4,3,205
0,6,214~0,8,214
7,3,225~8,3,225
3,9,84~4,9,84
7,7,40~9,7,40
0,0,201~0,3,201
1,6,235~1,7,235
9,4,74~9,8,74
5,3,180~5,5,180
6,7,167~9,7,167
2,4,329~2,7,329
9,0,186~9,1,186
7,3,139~7,6,139
1,9,321~5,9,321
1,6,248~3,6,248
0,7,64~0,7,65
0,1,256~0,2,256
2,1,308~2,3,308
5,7,355~7,7,355
3,5,164~5,5,164
0,3,96~1,3,96
4,6,121~4,9,121
8,4,14~8,6,14
8,4,94~9,4,94
3,0,320~7,0,320
0,3,169~0,4,169
4,1,266~4,4,266
2,5,313~5,5,313
2,7,207~4,7,207
6,1,286~6,3,286
1,9,269~3,9,269
0,3,46~2,3,46
0,0,326~0,2,326
3,8,192~4,8,192
9,3,171~9,5,171
3,5,219~3,5,221
3,7,86~4,7,86
2,7,12~2,9,12
5,1,16~6,1,16
3,8,52~3,8,54
2,5,95~3,5,95
3,4,199~3,6,199
1,4,150~2,4,150
1,3,151~1,4,151
3,1,130~3,3,130
1,1,335~1,1,338
8,2,43~8,2,45
8,7,284~9,7,284
2,1,239~2,1,241
0,1,258~1,1,258
0,0,324~0,0,324
6,3,302~6,3,304
0,5,197~0,7,197
4,9,339~5,9,339
7,4,310~7,7,310
0,4,3~0,7,3
5,3,136~5,6,136
5,7,3~5,7,4
0,5,166~3,5,166
6,1,239~9,1,239
4,1,181~4,3,181
2,5,130~4,5,130
7,7,68~7,7,71
7,0,16~7,0,17
8,2,40~8,4,40
9,4,112~9,4,115
0,7,224~1,7,224
3,2,343~3,5,343
3,5,336~3,8,336
5,1,59~9,1,59
3,1,298~5,1,298
9,6,159~9,8,159
0,5,49~2,5,49
3,7,135~3,9,135
8,2,153~8,4,153
4,6,163~4,6,165
5,2,317~5,3,317
4,3,26~6,3,26
9,4,176~9,6,176
7,7,272~7,7,274
1,5,201~3,5,201
2,1,23~2,4,23
8,6,37~9,6,37
7,0,201~7,0,204
6,3,287~7,3,287
8,6,226~8,6,228
8,3,247~8,5,247
7,1,223~7,4,223
4,0,265~6,0,265
0,0,203~2,0,203
5,1,119~6,1,119
2,3,128~4,3,128
1,1,306~3,1,306
5,4,249~6,4,249
3,1,53~4,1,53
6,2,93~8,2,93
6,0,9~6,2,9
1,7,290~3,7,290
1,3,54~1,3,54
3,0,208~5,0,208
8,9,13~9,9,13
5,5,151~5,7,151
5,7,125~7,7,125
4,8,283~8,8,283
3,5,177~6,5,177
9,3,187~9,5,187
5,7,248~7,7,248
3,5,73~4,5,73
6,6,71~9,6,71
1,3,196~1,6,196
6,8,139~6,8,140
9,7,98~9,9,98
3,5,4~3,6,4
3,2,57~4,2,57
7,8,156~7,8,159
7,0,142~7,3,142
0,6,296~0,7,296
9,4,39~9,6,39
7,4,35~8,4,35
5,9,335~5,9,336
6,7,57~9,7,57
1,0,321~2,0,321
9,2,109~9,4,109
5,3,125~7,3,125
4,3,302~4,4,302
3,8,214~5,8,214
2,0,128~3,0,128
8,7,101~8,7,103
4,3,311~6,3,311
8,0,96~8,2,96
8,2,49~8,4,49
2,1,183~5,1,183
5,5,342~5,6,342
4,3,344~6,3,344
0,5,136~0,7,136
5,8,106~5,8,108
1,6,1~1,7,1
5,1,89~5,3,89
5,3,5~5,4,5
7,7,15~9,7,15
7,0,283~7,1,283
1,1,26~3,1,26
1,7,302~3,7,302
1,8,205~2,8,205
8,0,199~8,2,199
3,5,198~3,7,198
2,5,85~2,7,85
4,8,53~4,9,53
1,5,5~1,7,5
0,0,281~2,0,281
2,2,302~5,2,302
7,9,99~9,9,99
4,9,359~6,9,359
2,4,5~2,7,5
3,4,137~5,4,137
0,3,336~2,3,336
9,3,182~9,5,182
0,6,37~2,6,37
1,5,172~4,5,172
6,4,51~6,6,51
1,9,289~3,9,289
3,6,285~5,6,285
8,0,50~8,0,52
5,2,228~8,2,228
2,2,117~4,2,117
2,8,50~4,8,50
3,5,205~4,5,205
0,3,252~0,4,252
7,0,115~9,0,115
6,8,179~9,8,179
3,6,226~3,9,226
6,0,166~9,0,166
1,3,279~1,6,279
7,9,226~9,9,226
5,2,156~5,3,156
4,9,1~6,9,1
6,3,79~9,3,79
5,2,107~5,4,107
5,8,5~5,9,5
5,2,287~8,2,287
0,5,133~0,7,133
4,6,38~6,6,38
0,2,82~2,2,82
7,8,164~7,8,165
4,2,293~4,2,295
7,3,258~9,3,258
0,0,278~2,0,278
0,2,89~2,2,89
2,5,234~2,6,234
1,2,324~3,2,324
9,3,159~9,3,162
4,5,257~4,6,257
0,8,199~0,8,199
1,8,3~1,8,4
9,0,122~9,2,122
1,5,225~4,5,225
3,3,183~5,3,183
3,2,182~3,4,182
0,7,292~0,9,292
2,5,175~4,5,175
3,6,67~5,6,67
3,3,92~3,5,92
1,5,269~3,5,269
3,0,214~6,0,214
3,2,263~4,2,263
3,4,66~3,6,66
3,4,247~3,4,249
0,4,197~3,4,197
3,5,248~6,5,248
0,1,124~2,1,124
9,6,189~9,8,189
3,6,208~3,8,208
5,7,7~5,8,7
8,6,187~8,6,190
4,4,37~5,4,37
3,0,299~3,2,299
4,0,48~4,0,50
3,2,310~6,2,310
3,2,291~5,2,291
0,4,39~0,5,39
3,1,232~5,1,232
3,9,315~5,9,315
5,1,18~7,1,18
5,3,247~7,3,247
2,1,53~2,3,53
0,0,3~0,1,3
0,1,233~2,1,233
6,1,191~8,1,191
2,7,26~2,7,26
6,1,3~6,4,3
3,1,303~4,1,303
7,6,142~9,6,142
3,5,268~3,7,268
5,3,341~7,3,341
2,3,40~5,3,40
1,5,310~3,5,310
7,0,329~7,2,329
0,3,23~0,5,23
5,6,164~5,9,164
9,4,276~9,5,276
5,3,122~5,5,122
3,8,102~6,8,102
4,1,217~4,3,217
5,5,276~5,5,277
0,3,243~2,3,243
1,2,346~4,2,346
8,3,254~8,3,257
4,9,87~8,9,87
4,9,275~6,9,275
1,3,249~1,6,249
1,3,284~1,6,284
7,2,307~7,6,307
9,0,175~9,2,175
6,5,179~8,5,179
1,1,152~1,3,152
7,4,275~7,5,275
1,2,302~1,2,303
1,6,148~1,7,148
0,4,299~0,4,300
6,6,121~6,9,121
9,4,186~9,7,186
2,1,200~2,2,200
0,4,260~1,4,260
7,0,51~7,3,51
2,4,265~2,4,268
6,2,157~6,4,157
9,2,1~9,3,1
5,8,154~6,8,154
1,6,89~1,8,89
7,4,168~9,4,168
9,2,290~9,5,290
1,4,68~3,4,68
1,6,349~3,6,349
2,7,2~5,7,2
2,7,223~6,7,223
2,4,176~2,6,176
7,8,39~7,8,41
2,8,333~2,8,334
4,6,353~4,8,353
7,6,124~7,9,124
8,5,15~8,5,15
2,4,93~2,7,93
6,0,187~6,2,187
4,2,340~6,2,340
0,2,138~0,3,138
2,5,180~4,5,180
4,4,30~5,4,30
2,3,42~2,6,42
4,0,14~4,3,14
4,9,165~6,9,165
2,2,321~3,2,321
6,1,141~6,4,141
2,2,179~2,4,179
2,9,57~3,9,57
3,5,91~3,7,91
6,5,99~6,8,99
6,4,254~8,4,254
3,1,171~3,1,174
5,5,69~8,5,69
6,7,83~6,9,83
4,0,117~6,0,117
1,0,50~3,0,50
3,0,328~3,0,329
8,5,344~8,7,344
7,3,309~9,3,309
4,4,236~7,4,236
6,7,169~9,7,169
6,6,295~6,8,295
2,1,75~2,3,75
0,7,190~2,7,190
1,9,138~3,9,138
8,6,289~9,6,289
6,5,255~8,5,255
7,3,281~7,5,281
1,2,165~1,2,168
3,7,201~3,7,204
5,6,121~5,8,121
0,2,259~0,3,259
3,3,70~3,4,70
1,5,51~1,7,51
0,8,215~1,8,215
9,6,144~9,9,144
2,1,249~2,2,249
6,6,151~6,9,151
4,7,143~4,9,143
0,9,291~2,9,291
5,7,306~5,8,306
0,3,235~0,6,235
4,2,43~4,3,43
4,5,183~5,5,183
9,3,39~9,3,41
0,2,318~2,2,318
6,3,320~9,3,320
0,1,95~0,3,95
7,2,162~9,2,162
3,0,233~3,2,233
4,0,267~6,0,267
2,9,272~6,9,272
4,4,126~4,5,126
3,8,9~5,8,9
1,9,323~3,9,323
8,2,318~8,3,318
4,0,24~6,0,24
6,5,326~6,8,326
5,3,315~8,3,315
6,8,173~9,8,173
1,4,291~1,7,291
4,7,331~5,7,331
4,8,194~8,8,194
7,5,162~7,8,162
5,7,137~7,7,137
6,3,289~8,3,289
4,1,76~4,3,76
2,6,198~2,8,198
3,1,83~3,4,83
9,6,229~9,8,229
1,1,254~1,3,254
7,0,243~7,3,243
3,5,159~3,6,159
0,0,51~2,0,51
4,7,336~4,8,336
7,0,45~7,3,45
1,3,138~4,3,138
1,9,97~1,9,97
3,2,13~3,4,13
1,9,290~3,9,290
5,0,13~8,0,13
0,7,93~0,9,93
5,4,282~7,4,282
6,6,297~6,8,297
6,7,77~6,7,79
4,5,129~4,7,129
2,1,224~3,1,224
6,5,49~8,5,49
7,0,158~7,1,158
1,3,208~4,3,208
3,6,280~3,8,280
0,3,302~1,3,302
4,2,190~5,2,190
2,1,169~4,1,169
0,4,72~1,4,72
9,8,217~9,8,219
9,3,186~9,3,186
5,0,245~7,0,245
5,7,279~7,7,279
9,1,264~9,3,264
7,0,275~8,0,275
8,0,314~9,0,314
4,8,134~6,8,134
5,2,140~5,4,140
2,5,340~3,5,340
8,5,70~9,5,70
6,5,100~7,5,100
7,4,276~7,4,276
3,4,74~5,4,74
4,4,53~7,4,53
6,1,276~6,3,276
4,3,153~4,5,153
4,1,16~4,1,18
0,4,56~2,4,56
4,8,41~6,8,41
5,5,321~5,8,321
6,8,195~6,9,195
6,3,37~8,3,37
6,5,169~9,5,169
6,1,284~6,3,284
3,6,162~6,6,162
4,3,2~6,3,2
0,5,164~2,5,164
7,6,76~8,6,76
4,4,56~4,7,56
0,1,131~0,4,131
0,3,333~0,5,333
4,2,78~4,5,78
5,6,175~5,6,177
5,0,58~7,0,58
3,3,220~6,3,220
0,7,227~2,7,227
6,0,278~8,0,278
3,1,200~5,1,200
1,0,167~2,0,167
2,8,201~4,8,201
2,3,47~2,3,48
0,0,237~3,0,237
5,5,328~7,5,328
6,4,76~6,6,76
6,8,337~6,8,339
4,3,126~7,3,126
8,4,257~9,4,257
7,2,301~8,2,301
3,5,59~4,5,59
6,5,72~6,5,73
2,4,128~4,4,128
6,6,246~6,8,246
4,7,131~4,9,131
7,1,322~9,1,322
2,3,44~2,4,44
8,6,177~9,6,177
6,0,3~6,0,6
3,8,303~5,8,303
2,7,265~2,7,267
3,4,335~3,7,335
2,4,95~2,4,98
6,0,160~6,3,160
7,6,174~9,6,174
3,5,17~3,6,17
2,5,147~2,5,149
6,0,35~6,3,35
5,5,264~5,8,264
0,3,297~0,5,297
4,7,135~6,7,135
5,1,62~8,1,62
3,2,289~5,2,289
0,8,257~3,8,257
5,0,263~5,3,263
7,7,220~7,9,220
1,9,5~1,9,5
1,4,53~3,4,53
8,6,13~8,8,13
8,6,74~8,7,74
4,8,198~7,8,198
9,0,321~9,3,321
1,8,202~2,8,202
3,3,36~3,5,36
7,4,93~8,4,93
2,5,18~2,6,18
7,6,153~7,8,153
6,3,118~7,3,118
4,4,346~4,6,346
7,5,121~7,6,121
5,5,256~5,7,256
3,6,218~3,7,218
4,3,150~4,5,150
4,3,47~6,3,47
5,1,324~5,1,327
5,6,13~5,7,13
5,5,184~8,5,184
1,7,85~1,7,87
6,1,115~6,1,117
4,5,119~5,5,119
1,6,246~1,7,246
0,2,92~1,2,92
3,8,195~3,9,195
3,2,293~3,4,293
6,2,2~8,2,2
7,4,332~7,6,332
1,7,206~1,9,206
5,4,339~5,5,339
2,0,184~2,1,184
0,3,20~3,3,20
1,2,169~1,3,169
4,2,258~4,5,258
4,8,276~7,8,276
1,2,85~1,2,87
1,6,224~4,6,224
6,6,298~6,6,301
0,2,43~2,2,43
6,3,52~8,3,52
8,2,114~8,2,114
5,7,11~5,9,11
8,8,291~8,9,291
5,4,70~5,7,70
2,8,184~4,8,184
1,4,351~3,4,351
6,8,298~9,8,298
2,3,73~6,3,73
6,2,238~6,4,238
3,2,349~3,2,352
0,7,222~2,7,222
1,3,48~1,5,48
9,0,130~9,2,130
5,6,139~5,6,139
2,7,282~5,7,282
0,6,238~2,6,238
9,4,183~9,7,183
1,4,341~4,4,341
6,1,143~6,3,143
1,7,286~3,7,286
0,6,232~2,6,232
3,3,169~3,5,169
2,6,295~4,6,295
3,0,51~3,3,51
8,3,324~8,4,324
7,7,150~9,7,150
9,8,22~9,9,22
8,4,76~9,4,76
2,8,208~2,9,208
9,6,180~9,6,182
3,6,69~3,8,69
0,4,25~0,4,28
7,0,252~9,0,252
2,2,199~2,4,199
2,0,158~5,0,158
0,9,4~2,9,4
5,6,268~7,6,268
4,3,66~4,5,66
1,5,37~3,5,37
3,1,133~4,1,133
0,3,295~0,5,295
1,8,90~1,8,92
4,5,230~4,6,230
8,6,68~8,8,68
4,1,227~4,2,227
5,4,230~7,4,230
0,5,236~0,7,236
2,0,215~4,0,215
4,2,305~7,2,305
9,0,188~9,1,188
6,1,306~8,1,306
8,5,194~8,6,194
3,3,350~3,4,350
0,2,254~0,5,254
2,9,81~4,9,81
5,6,169~5,9,169
5,2,272~5,2,273
4,3,299~6,3,299
0,4,168~0,6,168
2,6,223~4,6,223
0,4,132~0,5,132
5,0,57~5,2,57
9,5,164~9,8,164
6,6,222~9,6,222
3,3,212~3,3,214
6,8,287~8,8,287
4,6,270~4,8,270
2,4,162~5,4,162
1,1,14~1,3,14
4,1,5~4,3,5
9,1,4~9,2,4
5,0,49~8,0,49
4,0,313~4,1,313
9,4,179~9,7,179
4,6,334~4,8,334
4,7,224~4,9,224
8,7,49~8,8,49
8,5,172~8,5,173
9,1,42~9,3,42
1,1,155~1,3,155
6,4,9~6,8,9
2,7,87~2,9,87
8,5,206~8,8,206
0,3,82~2,3,82
0,8,309~2,8,309
0,0,53~1,0,53
9,6,100~9,7,100
3,6,83~3,7,83
1,6,145~1,9,145
6,1,111~9,1,111
1,7,188~2,7,188
0,0,323~2,0,323
1,7,257~3,7,257
6,2,255~9,2,255
3,6,297~3,8,297
7,5,107~8,5,107
0,6,195~4,6,195
5,4,78~5,6,78
0,0,213~0,3,213
1,4,262~3,4,262
8,5,329~8,7,329
7,6,342~9,6,342
3,7,17~6,7,17
5,1,189~6,1,189
3,1,39~3,3,39
4,8,215~6,8,215
4,5,315~6,5,315
8,1,18~9,1,18
4,6,323~6,6,323
2,4,46~2,6,46
2,4,100~2,4,102
7,6,49~7,7,49
4,0,21~4,2,21
5,2,104~8,2,104
3,0,156~3,3,156
7,1,259~7,2,259
5,3,44~8,3,44
4,5,82~4,7,82
8,1,102~8,3,102
2,4,223~4,4,223
7,2,240~9,2,240
1,0,340~4,0,340
7,3,232~7,3,233
5,0,11~7,0,11
0,3,303~1,3,303
4,3,34~7,3,34
3,4,339~3,6,339
1,4,93~1,6,93
6,8,219~8,8,219
1,5,1~1,5,4
6,1,153~6,1,154
5,8,68~7,8,68
1,3,343~1,5,343
5,2,117~6,2,117
7,6,179~8,6,179
0,2,1~0,5,1
9,2,330~9,3,330
1,3,144~3,3,144
6,0,193~6,1,193
2,5,3~2,5,4
3,7,292~3,8,292
1,0,170~1,0,173
1,0,40~1,3,40
7,7,67~9,7,67
4,4,250~4,6,250
3,0,265~3,2,265
3,7,308~5,7,308
1,3,71~1,5,71
4,9,291~6,9,291
0,3,240~0,6,240
6,2,159~7,2,159
4,3,10~6,3,10
4,5,318~6,5,318
2,3,330~5,3,330
6,2,338~6,3,338
4,7,197~4,9,197
1,2,300~1,4,300
2,3,202~4,3,202
7,4,33~7,6,33
4,4,154~5,4,154
9,2,261~9,4,261
5,2,204~5,2,206
3,8,86~6,8,86
2,6,135~2,8,135
1,9,241~2,9,241
5,1,194~7,1,194
3,0,8~6,0,8
0,7,310~0,9,310
4,2,298~4,4,298
3,3,312~5,3,312
2,2,79~2,2,79
0,6,286~0,8,286
4,2,315~6,2,315
8,8,15~8,8,17
6,0,200~6,1,200
3,3,307~6,3,307
9,4,343~9,6,343
1,9,96~1,9,96
5,5,41~7,5,41
3,5,188~3,9,188
7,4,112~7,4,113
0,0,137~0,4,137
1,2,52~1,2,55
2,0,187~2,1,187
4,5,75~4,5,76
1,0,205~1,0,206
6,0,281~7,0,281
5,9,170~5,9,171
2,4,226~4,4,226
8,2,109~8,3,109
1,3,259~1,3,261
5,8,309~5,9,309
5,0,25~5,2,25
8,2,197~8,5,197
2,0,218~2,0,220
5,2,229~6,2,229
7,4,225~7,5,225
8,2,112~8,2,113
6,3,250~6,4,250
9,1,183~9,3,183
7,3,242~9,3,242
2,5,83~2,7,83
2,5,87~4,5,87
4,5,278~4,5,280
5,7,47~8,7,47
7,7,289~7,9,289
4,1,229~4,3,229
5,3,76~9,3,76
3,4,314~3,6,314
2,0,130~5,0,130
7,1,116~7,3,116
1,6,328~4,6,328
4,7,183~7,7,183
2,5,213~4,5,213
6,1,163~9,1,163
3,4,216~3,6,216
0,9,90~3,9,90
2,1,247~2,4,247
3,6,267~3,9,267
4,6,182~6,6,182
0,5,75~0,6,75
2,4,25~3,4,25
6,0,116~9,0,116
7,0,113~7,3,113
4,9,276~5,9,276
9,3,244~9,5,244
3,7,132~5,7,132
5,9,17~5,9,17
2,3,145~4,3,145
6,2,312~8,2,312
4,4,34~4,7,34
4,2,308~7,2,308
9,7,4~9,9,4
8,7,176~8,9,176
3,4,82~3,6,82
6,5,245~6,8,245
4,4,71~4,5,71
7,1,21~7,1,22
7,7,60~7,9,60
9,4,118~9,6,118
9,2,107~9,4,107
6,1,170~9,1,170
5,5,116~5,7,116
5,1,13~5,4,13
4,1,333~4,2,333
6,4,33~6,5,33
6,4,244~6,6,244
4,0,317~4,1,317
2,6,59~2,6,61
0,1,78~2,1,78
2,3,270~2,4,270
4,4,263~6,4,263
9,0,123~9,1,123
1,6,330~1,7,330
1,3,313~1,5,313
7,4,152~9,4,152
4,3,273~6,3,273
6,6,145~8,6,145
7,1,256~9,1,256
3,9,312~6,9,312
7,5,73~7,7,73
5,8,22~8,8,22
7,2,11~9,2,11
5,3,29~5,5,29
5,0,203~5,2,203
3,4,75~5,4,75
8,2,245~8,5,245
9,0,165~9,2,165
4,2,225~7,2,225
5,4,111~5,6,111
0,5,252~2,5,252
6,4,8~7,4,8
3,0,29~3,3,29
8,1,112~8,1,115
7,4,97~9,4,97
0,0,211~3,0,211
0,3,257~0,6,257
2,9,356~4,9,356
4,2,185~5,2,185
7,0,200~7,3,200
1,5,236~1,7,236
1,6,146~3,6,146
4,4,109~6,4,109
1,5,296~1,6,296
1,2,268~3,2,268
9,1,258~9,1,260
9,3,154~9,3,156
1,5,141~4,5,141
4,3,12~4,5,12
4,1,15~4,4,15
1,1,37~2,1,37
6,2,210~6,3,210
2,0,236~2,2,236
6,5,185~7,5,185
5,1,338~5,3,338
6,5,5~6,6,5
4,4,70~4,6,70
1,6,226~2,6,226
9,1,295~9,1,297
4,4,306~5,4,306
5,3,87~7,3,87
3,5,257~3,6,257
5,1,270~5,4,270
2,4,133~2,7,133
3,0,47~3,2,47
9,6,147~9,8,147
1,1,167~3,1,167
2,4,35~2,6,35
1,1,70~1,4,70
9,3,167~9,5,167
7,5,102~7,5,104
4,1,287~6,1,287
4,4,120~4,6,120
2,1,152~2,4,152
5,0,165~5,4,165
8,1,280~9,1,280
1,0,322~3,0,322
8,4,277~8,5,277
6,8,135~6,8,136
5,0,27~5,2,27
5,3,184~7,3,184
7,1,20~7,4,20
4,4,156~4,4,159
5,7,78~5,9,78
7,4,190~7,5,190
2,8,14~2,9,14
2,4,348~4,4,348
4,8,27~6,8,27
9,4,228~9,6,228
4,3,155~7,3,155
0,7,295~0,8,295
6,6,24~6,9,24
7,2,5~7,4,5
6,7,3~8,7,3
2,5,178~5,5,178
8,1,171~8,2,171
2,6,21~2,8,21
7,2,229~7,3,229
9,4,185~9,8,185
4,5,263~4,8,263
3,7,278~5,7,278
3,3,210~3,5,210
8,7,170~8,7,173
5,8,249~5,8,251
2,5,15~4,5,15
4,3,335~7,3,335
1,0,99~3,0,99
5,4,267~5,6,267
2,5,330~2,5,332
8,9,17~8,9,20
4,6,148~6,6,148
5,8,296~7,8,296
0,5,281~2,5,281
5,7,166~7,7,166
8,6,99~8,8,99
1,6,82~1,7,82
3,4,6~3,6,6
9,7,335~9,7,337
4,4,222~4,6,222
2,1,339~5,1,339
6,3,171~6,5,171
2,8,22~2,8,24
2,7,332~2,9,332
5,3,259~5,7,259
2,0,326~4,0,326
3,5,88~3,5,89
3,5,256~3,8,256
4,5,266~4,6,266
4,3,278~6,3,278
3,1,3~3,1,5
1,6,198~1,7,198
5,1,58~5,3,58
4,5,79~5,5,79
0,3,130~2,3,130
1,2,32~3,2,32
3,0,226~3,2,226
2,0,54~3,0,54
0,3,199~0,5,199
4,1,189~4,3,189
0,0,329~0,2,329
8,2,251~8,4,251
0,3,134~0,4,134
7,8,224~8,8,224
1,6,39~1,7,39
7,9,224~8,9,224
4,0,45~4,3,45
4,0,216~6,0,216
5,3,240~8,3,240
4,5,200~4,5,202
6,2,105~9,2,105
8,1,108~8,3,108
0,8,308~4,8,308
7,0,332~9,0,332
1,0,20~3,0,20
5,0,326~7,0,326
6,3,84~9,3,84
6,6,93~6,8,93
0,9,286~4,9,286
3,1,221~3,3,221
9,1,332~9,3,332
7,4,279~7,6,279
5,9,201~7,9,201
1,5,193~1,7,193
4,7,138~6,7,138
4,6,268~4,8,268
1,1,52~2,1,52
4,3,281~6,3,281
6,6,271~6,6,272
2,1,46~4,1,46
7,5,329~7,5,331
5,3,41~5,4,41
4,0,337~4,2,337
9,0,125~9,2,125
7,3,274~7,6,274
2,5,69~3,5,69
6,5,46~6,7,46
3,3,196~3,6,196
1,3,143~1,6,143
0,6,63~0,7,63
9,5,101~9,9,101
3,1,219~4,1,219
7,1,237~7,4,237
2,0,204~5,0,204
5,8,167~7,8,167
8,4,276~8,6,276
1,1,302~4,1,302
0,4,332~2,4,332
2,2,271~3,2,271
5,5,282~6,5,282
5,6,335~5,8,335
9,3,245~9,3,246
8,6,339~8,8,339
1,8,189~3,8,189
4,6,275~4,8,275
7,4,100~7,4,102
2,3,57~2,6,57
6,5,247~7,5,247
3,6,340~3,8,340
5,1,197~8,1,197
7,5,38~7,8,38
5,2,86~6,2,86
5,2,184~7,2,184
2,0,161~2,3,161
5,2,292~5,3,292
3,4,349~5,4,349
7,9,63~7,9,65
6,5,254~8,5,254
5,3,32~5,5,32
5,6,334~5,8,334
4,7,327~7,7,327
7,7,328~9,7,328
2,3,304~4,3,304
5,2,189~7,2,189
4,1,102~7,1,102
2,8,211~3,8,211
4,7,292~7,7,292
5,3,187~5,4,187
1,1,25~3,1,25
1,1,186~3,1,186
8,7,181~8,9,181
1,7,205~3,7,205
6,6,8~8,6,8
9,0,250~9,2,250
6,2,154~6,5,154
6,1,99~8,1,99
7,7,281~9,7,281
5,1,181~5,3,181
9,4,42~9,4,44
5,3,173~7,3,173
5,6,332~5,9,332
2,5,156~4,5,156
6,3,339~8,3,339
2,2,21~3,2,21
4,4,198~4,6,198
1,3,11~1,4,11
2,2,315~2,5,315
0,4,38~0,8,38
6,8,66~8,8,66
7,2,140~7,5,140
2,9,238~4,9,238
5,7,170~5,8,170
8,5,5~8,7,5
6,2,50~8,2,50
6,7,286~6,9,286
5,3,113~5,5,113
6,7,104~6,9,104
8,6,192~8,8,192
7,8,20~9,8,20
7,0,312~9,0,312
5,2,252~5,4,252
1,4,147~1,5,147
5,6,218~5,7,218
1,6,293~2,6,293
5,7,119~5,8,119
1,6,301~1,8,301
1,2,170~1,2,172
3,1,333~3,3,333
5,2,262~5,5,262
9,9,1~9,9,3
8,6,75~8,6,75
2,9,54~4,9,54
9,0,83~9,2,83
1,4,33~4,4,33
4,7,285~4,9,285
8,2,259~8,3,259
5,6,354~5,7,354
7,7,184~7,9,184
3,7,284~3,9,284
6,7,4~7,7,4
3,1,342~5,1,342
4,2,261~4,3,261
8,0,308~8,0,310
1,7,81~4,7,81
2,6,229~4,6,229
1,2,164~2,2,164
6,6,334~6,8,334
0,4,217~0,6,217
4,7,337~4,7,337
3,4,14~3,8,14
3,0,206~5,0,206
3,7,142~5,7,142
1,2,145~1,4,145
3,8,255~6,8,255
2,2,180~4,2,180
5,3,333~7,3,333
6,5,236~6,8,236
4,2,208~6,2,208
3,5,233~3,8,233
4,0,297~4,2,297
3,6,331~3,6,333
2,7,48~2,7,50
4,3,216~4,5,216
4,7,258~4,9,258
6,0,323~6,2,323
0,9,3~3,9,3
6,9,221~8,9,221
6,7,96~9,7,96
2,6,24~2,7,24
3,5,308~5,5,308
4,5,273~4,7,273
0,4,73~0,6,73
6,5,331~6,6,331
9,1,292~9,4,292
7,9,16~8,9,16
3,7,235~3,9,235
7,1,39~7,2,39
2,2,227~2,4,227
2,6,178~4,6,178
1,2,51~1,3,51
4,6,181~4,9,181
5,4,133~5,6,133
3,0,316~5,0,316
2,5,353~2,8,353
2,8,10~4,8,10
0,6,62~2,6,62
0,4,244~3,4,244
6,8,201~7,8,201
7,5,155~7,8,155
6,8,204~8,8,204
0,7,357~3,7,357
0,7,191~0,9,191
6,5,239~6,5,241
1,7,288~1,8,288
3,7,166~3,9,166
6,3,82~7,3,82
1,4,296~1,4,298
2,5,144~4,5,144
5,5,153~5,6,153
1,5,276~4,5,276
1,6,299~1,8,299
6,7,338~7,7,338
0,2,35~1,2,35
8,2,15~8,2,17
7,5,273~9,5,273
4,2,254~4,5,254
9,6,286~9,8,286
1,3,309~3,3,309
7,6,158~9,6,158
3,3,147~3,5,147
2,6,343~4,6,343
3,3,23~5,3,23
6,8,340~9,8,340
4,6,221~6,6,221
1,1,129~2,1,129
9,1,43~9,3,43
7,1,103~9,1,103
2,5,219~2,7,219
5,8,280~7,8,280
3,6,163~3,9,163
7,2,82~9,2,82
2,1,311~4,1,311
4,3,148~6,3,148
2,0,230~2,3,230
3,1,121~5,1,121
4,2,183~5,2,183
2,7,354~2,9,354
6,0,286~8,0,286
7,2,79~9,2,79
8,6,10~8,9,10
2,0,127~2,2,127
4,2,299~7,2,299
8,1,78~8,3,78
0,6,2~0,8,2
3,1,79~3,1,81
5,7,62~8,7,62
2,2,54~5,2,54
2,4,344~2,5,344
2,1,123~4,1,123
4,5,260~4,7,260
6,2,258~6,2,260
8,1,253~8,2,253
8,4,198~8,5,198
6,2,249~9,2,249
2,7,17~2,9,17
8,5,65~8,7,65
5,5,344~5,8,344
5,1,127~5,1,127
8,6,278~8,6,282
7,5,74~7,7,74
6,7,122~6,9,122
5,5,2~8,5,2
2,7,293~2,7,295
2,0,275~2,0,277
1,4,24~3,4,24
3,5,223~5,5,223
5,1,5~7,1,5
1,7,146~1,9,146
0,3,354~2,3,354
8,5,186~8,7,186
4,1,221~6,1,221
3,7,330~5,7,330
7,6,58~7,6,60
5,6,253~5,9,253
1,4,194~1,5,194
6,2,10~8,2,10
9,6,72~9,7,72
6,3,127~8,3,127
5,0,215~7,0,215
1,1,81~1,4,81
4,8,40~6,8,40
4,5,252~6,5,252
4,9,256~5,9,256
2,6,351~5,6,351
4,0,166~5,0,166
6,0,274~9,0,274
0,5,204~1,5,204
3,5,65~6,5,65
3,2,86~4,2,86
4,7,277~6,7,277
0,8,192~0,8,195
1,3,8~4,3,8
4,4,326~4,6,326
8,0,46~8,3,46
5,4,31~7,4,31
1,6,95~1,9,95
8,9,183~8,9,185
0,5,284~0,8,284
5,2,164~5,4,164
5,9,12~5,9,14
6,2,280~6,5,280
6,9,288~6,9,290
5,6,119~7,6,119
8,3,322~8,4,322
1,6,42~1,8,42
0,2,238~3,2,238
1,1,334~3,1,334
7,3,176~8,3,176
1,3,352~1,4,352
1,3,139~3,3,139
9,8,187~9,9,187
9,2,324~9,2,327
5,1,277~7,1,277
8,4,274~8,5,274
4,0,205~5,0,205
7,8,343~8,8,343
5,4,273~5,6,273
2,2,16~5,2,16
5,7,48~5,8,48
9,0,118~9,1,118
7,9,229~7,9,231
3,8,217~6,8,217
3,6,2~6,6,2
3,7,60~6,7,60
3,2,168~6,2,168
5,2,329~5,5,329
9,3,36~9,6,36
5,0,196~6,0,196
7,1,278~9,1,278
0,0,42~3,0,42
4,8,247~7,8,247
2,0,19~2,3,19
7,7,332~9,7,332
5,5,6~5,8,6
6,7,6~6,9,6
6,3,246~6,4,246
2,3,1~5,3,1
8,6,207~8,8,207
9,3,151~9,5,151
7,6,35~9,6,35
2,5,161~4,5,161
5,2,85~8,2,85
9,2,294~9,4,294
6,3,150~9,3,150
3,4,139~3,5,139
2,6,47~2,9,47
0,6,64~0,6,66
4,2,188~7,2,188
4,1,113~6,1,113
8,0,127~9,0,127
2,0,229~2,2,229
8,4,119~9,4,119
7,1,54~7,2,54
5,5,137~6,5,137
6,1,290~8,1,290
6,2,139~6,3,139
2,3,163~2,3,165
6,3,308~6,5,308
7,4,55~7,7,55
2,0,44~2,1,44
4,4,305~4,5,305
0,0,140~0,1,140
5,0,115~5,3,115
3,2,334~5,2,334
3,0,44~3,2,44
4,4,62~4,6,62
8,6,36~8,8,36
5,1,15~9,1,15
0,1,308~1,1,308
0,3,10~3,3,10
6,5,12~9,5,12
0,4,294~3,4,294
3,9,19~5,9,19
2,7,331~2,7,331
0,7,239~0,7,241
5,5,67~7,5,67
1,0,320~1,2,320
7,6,163~9,6,163
0,7,139~0,9,139
6,2,90~6,6,90
7,2,35~7,3,35
0,6,211~3,6,211
2,9,88~3,9,88
1,4,13~1,4,14
1,5,90~4,5,90
5,1,36~7,1,36
6,1,106~6,3,106
5,7,293~6,7,293
9,6,331~9,9,331
4,7,77~5,7,77
4,0,156~7,0,156
0,3,251~1,3,251
6,1,152~7,1,152
2,2,120~4,2,120
4,1,332~4,3,332
7,7,2~7,9,2
0,5,200~0,7,200
3,0,154~6,0,154
4,4,122~4,4,125
2,6,186~2,8,186
5,6,173~8,6,173
2,2,151~2,4,151
5,9,318~8,9,318
6,5,150~6,7,150
8,6,223~8,8,223
2,1,118~2,3,118
5,4,79~5,4,83
1,3,156~1,4,156
4,2,116~5,2,116
7,2,289~9,2,289
6,7,337~8,7,337
0,0,93~0,3,93
4,6,130~6,6,130
5,7,139~5,9,139
0,6,230~3,6,230
2,3,214~2,5,214
3,1,124~5,1,124
4,3,283~4,6,283
2,0,273~6,0,273
3,1,78~5,1,78
8,0,169~8,2,169
0,0,188~3,0,188
2,8,300~4,8,300
6,3,40~7,3,40
6,3,175~6,4,175
5,5,188~7,5,188
7,9,319~7,9,322
5,1,48~8,1,48
5,2,210~5,5,210
0,5,315~1,5,315
2,3,347~2,6,347
3,6,72~3,8,72
0,0,96~3,0,96
5,7,75~7,7,75
2,7,264~4,7,264
4,9,2~6,9,2
6,8,216~9,8,216
2,6,217~5,6,217
4,8,277~6,8,277
0,3,239~0,5,239
8,0,307~8,2,307
1,1,50~1,4,50
1,4,241~1,7,241
6,4,111~7,4,111
6,6,170~6,8,170
0,6,171~0,6,175
1,7,244~4,7,244
1,1,34~1,4,34
4,6,37~4,8,37
7,2,242~9,2,242
8,2,20~8,5,20
6,4,227~9,4,227
8,0,172~9,0,172
7,1,234~7,3,234
5,9,198~6,9,198
9,5,245~9,6,245
6,0,270~8,0,270
6,5,178~6,7,178
6,4,43~6,5,43
5,7,195~5,8,195
2,2,131~3,2,131
4,7,79~4,9,79
6,6,80~6,9,80
4,2,151~4,4,151
4,8,256~6,8,256
7,9,17~7,9,17
7,0,1~8,0,1
3,6,80~5,6,80
6,5,64~6,8,64
5,5,173~7,5,173
8,4,326~8,4,328
7,2,12~8,2,12
7,4,270~7,7,270
3,4,282~3,6,282
3,8,186~5,8,186
0,3,57~0,4,57
6,0,151~6,3,151
4,3,18~6,3,18
6,4,92~9,4,92
1,2,257~1,5,257
6,9,225~7,9,225
6,9,223~8,9,223
7,3,239~9,3,239
2,1,76~2,3,76
6,3,222~8,3,222
4,7,61~4,8,61
3,8,305~3,8,307
5,5,338~5,6,338
5,0,322~5,4,322
9,3,110~9,3,113
0,8,197~2,8,197";