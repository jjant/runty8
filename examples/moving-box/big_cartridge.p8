pico-8 cartridge // http://www.pico-8.com
version 41
__lua__
--dungeon!
--by deklaswas

--initialization
function _init()
a=0
music(39)
b={
c=7,
d=105
}
e=1
f=1
g=0
h=0
i=0
j=true
k=false
l=0
m=0
n=0
o=0
p=3
q=3
r=0
s=0
u=22
v=22
w=0
x=5
y=9
z=0
ba=1.5
bb=4
bc=""
bd=0
be=256
bf=0
bg=0
bh=""
bi=0
bj=split("health potion,blue cape,green cape,power cape,,,golden armor,obsidian armor,spider armor,,sword,bow and arrow,boomerang,ball and chain")
bk=split("restores one heart,lets you double jump,makes you run faster,multiplies damage by 2,,,gives you an extra heart,lets you touch fire,lets you climb walls\navoid spider damage,,press x to slash\nuse to attack enemies,press x to shoot\nhalf the damage of a sword,press x to throw\nsame damage as a sword,press x to swing\nmore damage than a sword")
bl=split("there is a shop at level 12\n\nwhere you can buy upgrades,if an enemy is on fire...\n\n\nyou probably shouldn't touch it,be on the lookout for treasure,beware of mimic chests!\n\nthey're scary,you can not jump on spiders,gold armor is known for\n\nfalling apart easily,before entering a portal\n\nwatch out where you will end up,obsidian armor does not protect\n\nyou from the spooky green fire,this tip is\n\ntotally useless")
bm=split("oh brave knight, you\nhave finally rescued me\nfrom this dungeon!|i sure hope it wasn't\nany trouble for you!|it's... been a long day.||well, i'm so glad you\ncould make it!|now... would you like\na kiss?|uh... what? no thanks.|...why not? it's my way\nof thanking you!|there's... no need to\nthank me.|do you not like me?|please lady, we just\nmet, and i wanna\ngo home.|but i thought you could\nbe my boyfriend after\nyou rescue me!|look, this is my job.\n\nto save princesses.\nthat's it.|i swear, this happens\nevery time.\n\ni don't want to date you.\ni'm sorry.|do you think i'm ugly?|listen, i'm gay,\nalright?|...what?|is that what you wanted\nto hear? i'm gay.|you're... gay?\n\nare you kidding me?|yes, i'm gay. why? is\nthere something wrong\nwith that?|but... you were so\nhandsome...|ok. does that change\nanything?|pardon me for\ninterrupting...|but i couldn't help\nnotice you two talking...|do you have anything\nto say?|...i hear there is a\nhandsome young prince\nwho is being held in a\ncastle in roguewind...|...in roguewind?|yes, a little ways\nnorth of here...|pardon me, but do you\nknow which way\nthe exit is?|...down the hall and\nto the right.|thank you!","|")
bn=0
bo=0
bp=0
bq=0
br=-1
bs=1
bt=256
bu=0
bv=384
bw=256
bx=0
by=1
bz=1000
ca=1000
cb=0
cc=0
cd=0
ce={}
cf={}
cg={}
ch={}
ci={}
function cj(ck)
ck=split(ck,"|")
local cl={
cm=ck[4],
cn=ck[5],
co=ck[6],
cp=ck[7],
schem=ck[1],
cr=ck[2],
cs=ck[3],
ct=ck[8]==2
}
add(ci,cl)
end
cu=0
cv={1}
cw=1
cx=1
cy={1}
cz={
"0;6,,,;79;24,15,31,18,53,5,47,12,6,3,,2,5,3;73;21,5,11,5,11,5,11,5,155,,,,,;76;18,,9,,5,,9,,5,11,5,11,5,11;77;51,9,38,11;119;85,5;120;132,7,42,5;230;135;231;136;246;151;247;152;245;150;248;153;232;169;|2|4.1|72|73|73|74",
"110;17;94;182;73;90,1,1,1,1,1,1,1,1,1;72;16,1,1,1,1,,,,,,,21,,66;76;181,1,1,1;75;165;80;87,,,77,,,;116;91,,66;81;110,15,1,15;120;60,119|4|4.1|72|73|73|74",
"73;80,,,,,40,,,,13,1,1,64;75;92;76;108;116;76,149,,,,,,,,,,;120;34,145,22;179;98,5,,13,5,10,4;181;110;110|1|4.1|72|73|72|74|2",
"72;85,,,,,,,,,,,65,,,,,,,,,,;73;69,65,1;80;81,,,,87,,,;116;53,17,75,,,84;119;24,4,,80,77;117;222;118;135,1;120;60,150;81;50,6,,,8,32,4,1,23,1,32,10,13,6;50,56,57,58,66,98,102,118,141,157,189,199,212,218|4|1.1|72|73|73|74|2",
"72;80,,,,,,51,,;73;86,10,1,1,7,4,5,1,22,1,1,1;75;98,2,53;76;28,,1,1,1,37,2,14,2,14,2,21,1,1,1;77;44,33,85,2,69;74;176,,,,,,66,,,,,,,,;120;36,167;81;39,,17,1,1,68,32,32;177;122;122|1|4.1|72|0|73|0|2",
"72;80,,,,,,,,,,,,,,,;74;160,,,,,,,,,,,,,,,;119;98,,,,,,,,,,,;180;151,4,;151,155,156|3|3|0|0|0|0|2",
"72;85,,,73,,,,,,5,,;73;92,1,1,1,1,32,1,1,1;75;178,2;76;103,91,2,14,2,14,2;77;26,75,18;116;69,156,2,2,,,,,,;120;44,70;81;35,,,,17;178;155;155|3|4.1|73|73|73|74",
"73;21,1,1,1,1,1,1,1,1,1;75;138;76;154,1,1,1,1;77;22,8,204;80;81,,,,66,,6,;116;122;117;116,32,1;118;129,32,32;120;35,165,4;81;40,2,2,27,2,2,2,27,2,2;176;58;177;230;179;60,28;60,88|1|4.1|73|73|73|74",
"72;86,2,2,2,,,45,24,,2,2,2,2,23;73;85,2,2,2,74,2,2,2;75;181;76;155,24,18,1,1;116;147,3,2,2;119;188;117;84,78,1,1,12,1,1;120;45,151,24;81;34,,15,,68,2,21,15,17;179;130;181;228;110;21;21|5|1.1|72|73|73|72",
"0;35,,,,,,,10,,,,,,,10,,,,,,;72;160,,,,,,,,,,,,,,,,,1,1,1;73;18,,,,,,,,,,,,5,11,5,11,5,11,5,11,5,11,5,,,,,,,,,,,;214;52;215;53;216;54;217;55;218;56;43;91;42;90;26;74;27;75;203;85;204;101;101|4|4.1|0|0|0|74|2",
"179;34;73;138,1,14,,,,,,;80;161,,,,,,;177;150;74;139,,,;110;185;72;247,,,,,,,,,-239,1,1,1,,,,,,,,,,,;120;194,-85;194,109|4|4|72|73|73|74",
"73;5,,,,,,70,15;0;246,,,;75;218,-197,5;76;234,-197,5,11,5,11,5,11,5,11,5,11,5;77;133,5;74;230,,,;80;81,,,;116;225,,,,,6,,,;117;100,1,1;81;39,,15,,15,,15,;177;216;120;162,11;162,173|1|1.1|72|72|72|74",
"73;0,,,,,1,1,1,1,1,,,,,,,1,57,14,,;75;172,28;76;188,1,12,4;77;232,4;74;160,,;80;91,,,;116;225,,,,,,,2,,,2,;119;17,,;120;44,69;81;34,15,2,15,74,28,37,,15,;110;21;21|3|1.1|72|73|72|74",
"73;16,15,,15,,15,,15,6,5,11,5,11,5,11,5,11,5,11,5,10,7,9,7,9,7,9,7;74;80,,,,,7,,,,,149,,,,,,,;81;167,,14,,,,13,,,,14,;180;229,,,,,;229,230,231,232,233,234|1|1.1|72|0|0|0|2",
"72;5,5,182,,,,,,5,,,,,,38,5;73;50,,,,5,,,,4,-16,29,-16,18,,,,,,5,,,,,,,15,,15,,15,,15,,15,,15;75;21,5,187,5;76;37,5,59,5,11,5;77;133,5,91,5;81;24,15,17,42,,9,,4,,,,7,,,;178;179;179;69;177;188;188|5|5.7|0|0|0|0",
"234;113;235;114;250;129;251;130;205;185;206;186;207;187;221;201;222;202;223;203;79;178,5,7,22,15,6,3;185;17;72;145,;73;85;75;71,3,3,70;120;195,3,7;127;161,,,,,,,,,;80;86,,,,,,,,,77,,,;86,87,88,89,90,91,92,93,94,171,172,173,174|2|4.1|72|73|73|74",
"0;246,,,;72;134,,,;73;133,5;75;149,5;76;165,5,11,5,11,5,11,5;77;229,5;80;69,,,,,,7,,11,;120;39,,107,9,38,11;81;34,3,,3,,3,5,11,41,,,;179;51,57,65;51,108,173|4|4.1|72|73|73|74|2",
"72;80,,,3,,,,,,,;75;21,10,42,3,20,102,39;76;37,1,1,32,7,1,90,1;77;102,5,10,23,84,15;80;83,;116;70,,,2,,150,,,,,2,,,,,,2;119;133;118;93;81;8,15,17,79,,,,42,2,12,27;179;217;217|1|5.7|72|76|76|74",
"72;122,,,,,,30,,;73;80,,15,,1,,,15,,1,,,15,,1,,,15,,1,,,15,;75;141;120;50,41,27,68;81;26,2,2,13,2,13,2,2,22,68,68;177;237;178;109;109|1|4.1|72|0|73|74",
"0;240,,,,;72;167,,,,,,,,;73;16,1,1,11,,4,5,11,,84,1,1,1,1;75;85,148;76;17,10,6,1,52,1,1,1;77;43,22;74;160,,,,;80;120,,,,,;116;53,98,,,,,,,,72,,;118;61,9;120;115,91;81;35,4,11,6,33,2,27,32,32;181;44;44|3|4.1|72|0|73|74",
"120;42,153,9;94;102;110;181;72;21,1,1,1,1,5,11,5,11,5,11,5,11,5,11,,,,,;119;17,,,;117;164;118;91,22,58;81;33,2,15,2,9,4,2,15,2,41,64|4|5.7|72|72|72|74",
"94;97,10;110;185;73;80,5,5,5,,5,5,5,,5,5,5,,5,5,5,,5,5,5,,,,,,,,3,,,,,,;80;86,,,;120;37,5,153,4;177;71;71|4|4|72|72|72|74",
"72;80,1,1,1,1,-49,1,1,1,1,-75,,,,,,,,,,,,65,,,,,,,,,,,,4;73;100,55;116;139;119;116;81;50,32,32,27,32,32;176;109;120;82,91;110;23;23|4|1.1|72|73|73|74",
"110;101,80,-160;72;85,,,,,,75,,,,,;73;84,7,,,,67,,,,7;80;81,,,89,,;120;34,91,69;34,125,194|4|1.1|73|73|73|74",
"0;0,15;73;19,9,5,,,9,,,3,13,3,13,2,,13,,65,,,11,,,3,11,5,11,5,11,5,11;80;163,,,,,,,,,;116;227,,,,,,,,,;120;115,9;178;158;179;78,-28;78,50|3|3|72|0|0|74|2",
"73;96,,4,,,,,,4,,49,5,5,5,,5,5,5,,5,5,5,,5,5,5,,5,5,5,,5,5,5;80;161,,,,2,,,,2,,,;120;130,11;81;21,2,2,13,2,2,11,2,2;177;88;178;148;94;177,10;177,187|5|1.1|72|72|72|72",
"0;245,,;72;86,,,,,70,,,,11,,24,15,15;73;85,1,63;76;104,1,1,1,1,1,1,1,1;77;21,5;80;97,,,;116;233,,,,,;120;134,61;81;59,17,17,1;177;72;72|4|3|73|73|73|74",
"73;10,,,,,,11,5,11,5,11,5,11,5,6,,,,,,5,65,5,,,,,,6,5,11,5,11,5,11,5,11,,,,,;80;161,,,;117;84;118;91,80;120;34,82,7,82;81;44,,15,;177;72;94;177;110;101;101|5|5.7|72|72|72|72",
"94;177,5,5;72;181,5,11,5,11,5,11,5;73;80,8,,,,,,,,,1,1,1,1,5,5,5;75;76;80;81,,,,,,;116;72,;119;23,;120;43,2,72,5;176;63;181;60;60|1|3|72|72|72|72",
"72;26,1,1,,,,,,34,66,77,,,,;73;68,,,14,1,1,1,6,10,6,10,,,,,,,1,1,1,1;76;86,1;77;118;0;11,,,,,1,1;116;81,41,25,2,,,,;81;36,,,51,,,8,5,10,15;176;205;205|5|4.1|72|72|72|73",
"72;89,,,,,;73;84,,76,,,,;75;169,5;76;185,5,11,5,11,5,11,5;80;166,,;117;88;118;86;120;123,2;81;42,2,2,4,32,32;177;76;94;186;182;23;23|4|1.1|72|73|72|74",
"0;240,,,13,-1,-1;73;17,13,3,13,2,,13,,129,,,,9,,,,4,9,7,9;75;64,15;76;80,15,,15,,15,,15,,15,,15;77;176,15;80;55,,45,,,,,,59,,,,,;120;70,3,140,5;81;23,,45,5,23,13,8,2,9,6,2,5;182;51,105;51,156|5|5.7|73|0|0|73",
"73;0,,,,,,5,6,10,1,12,,,,,,,,,,1,17,1,,,,,,,,,,,,37,,,,,,,,,,,,68;75;32,63,41;76;48,1,47,1,1,1,1,1,1;77;80,72,71;80;51,,;120;87,3,104,11;81;82,,,53,,15,;110;181;182;141;141|4|5.7|0|0|0|74",
"0;16,15,209,,13,;72;96,,15,14,,1,17,4,,,,,,,,4;73;3,,,5,,,7,9,4,,,,9,,,,,15,145,15,,,,11,,;75;64,15;77;192,15;80;54,,,,41,,,23,,,36,,,9,,;120;196,7;81;23,,41,1,6,,6,1,7,2,,2;177;148,7;178;150;150|5|5.7|0|76|76|73",
"0;0,15,240,-15;72;69,,,,,;73;,2,9,2,2,,13,,49,15,65,,,,,,5,,,,,,,15,,15,,,13,,2,13;75;32,15,132,9;76;48,15,132,9,7,9;77;64,15,99,2,7,2,38,9;80;114,;116;230,,,;120;126;81;107,,15,,15,;178;57;182;87,81;87,168|3|3|72|0|0|74",
"73;21,1,1,1,,,96,65,6,3,6;75;87,8;76;103,8,8,8,8,8;77;151,8;74;233,,,,,5,,,,;80;81,,,,38,,,62,,;120;115,65;81;41,4,30,30,4,30,30,4,30;177;229;182;58,98;58,156|1|5.7|73|73|73|0",
"0;96,1,1;72;81,64;73;82,,,,,27,,,,,,,,,,,;75;76,21,4,73,23,21;76;21,71,1,23,59,1,7,9,7,5,4;77;26,11,92,4,6,8;80;125,;116;102,,,,,,123,,,,2,,;120;194;81;19,15,17,5,,13,5,67,23,21;182;25;179;152;152|4|4|72|72|72|74",
"72;1,15,64,65,15,45,,,7,6,5,,,,;73;0,35,,,,,,,,,,26,,,,134,1;76;20,2,3,2,27,3;77;52,7,27,3;74;230,,,,;80;172,,;120;99,9;181;218;218|4|3|72|73|73|0|2",
"73;33,,,,,,,,,,,,,,115,,,,,,3,,,,,;75;18,2,2,3,2,2;77;49,13;80;115,,7,,43,,47,;120;118,3;81;67,2,5,2,119,2,5,2,6,2,7,2;176;142;178;228;181;236;182;55;55|3|3|73|73|73|74",
"72;0,,80,,,,170,;73;31,1,38,21,11,21,11,21,32,6,1;75;90;76;101,21,11,21,32,32;77;165,69;80;107,,,,61,,,;120;35,26,70,89;81;87,17,15,5,12,4,11,17,21,1;177;92,66;178;66;182;153,77;153,230|3|4.1|73|72|72|73|2",
"72;10,,,,,,10,15,15,15,15,8,,,5,8,7,8,7,8,7,8,7,8,15,15,15,15,12,,,;120;45,6,54,28,61,10;81;74,45,45;176;104;180;78;96;1,15,,15,,15,,15,97,15,,15,,15,,15;1,31,32,47,48,63,64,79,176,191,192,207,208,223,224,239|5|5.7|73|73|73|73",
"73;17,13,3,13,163,13,3,13;80;102,,,,58,,,,,,,,,;120;113,13,6,7;81;35,4,,4,7,4,,4,7,9,7,9,27,,15,,43,9,7,4,,4,7,4,,4;176;125;178;154;182;53,149;112;2,,,2,,,,2,,,229,,,2,,,,2,,;2,3,4,6,7,8,9,11,12,13,242,243,244,246,247,248,249,251,252,253|3|3|72|73|73|72",
"73;32,,13,,145,,13,;80;39,,29,,,,,,23,,,9,,,23,,,,,,23,,,9,,,23,,,,,;116;225,,,9,,;120;84,7,57,7;81;36,7,23,11,10,,15,,10,11,5,11,10,,15,;178;122,59;96;48,15,,15,,15,,15,,15,,15,,15,,15,,15;48,63,64,79,80,95,96,111,112,127,128,143,144,159,160,175,176,191|5|5.7|72|72|72|72",
"72;5,5,1,1,1,1,1,6,,,,,,5,,,,,50,,,,,,5,,,,,71,5;80;102,,,;120;35,160,3,3,3;81;17,2,2,6,,,,4,2,13,2,2,61,2,7,2,4,2,9,2,4,2,7,2;178;83;179;133,5;94;43;96;112,15,,15,,15;112;11,,,,237,,,;11,12,13,14,251,252,253,254|4|5.7|73|73|72|73",
"72;4,,,,,48,,43,,;73;160,3,,,7,,,,6,1,1,1,1,,,,,,;75;67,9;76;25,58,9,7,9,7,9,7,9;77;41,106,9;80;183,,,;116;157,,72,,,,,;120;194;81;18,15,17,15,17,15,8,9,7,1;177;169;178;85;180;149;181;40;40|4|4.1|73|96|96|74",
"72;0,,,,,,,,,8,1,1,1,3,,,11,3,39,,,,2,1,14,2,,,,,,,,7,2,14,1,1,1;75;141,39;76;196,1,1;116;230,,,,,;117;121;118;167;120;43,2,68,3,3;178;108;162;105,46;105,151|3|4.1|73|73|73|73",
"178;150;180;146;72;82,4,3,4,69,4,3,4;73;2,,,7,,,68,6,,6,67,6,,6,68,,,7,,;120;34,11,149,11;81;39,,15,,44,7,11,,,,11,2,,,,2,25,7,28,,15,;177;72;181;65;96;96,15,,15,,15,,15;112;6,,,,237,,,;6,7,8,9,246,247,248,249|4|1.1|72|72|72|72",
"74;135,;80;81,,,89,,;116;119,;81;45,13,3,10,,5,10,,12,7,7,2,7,9,7,1,12,,15,,5,13,3,1;176;125;125|1|3|112|73|73|112",
"73;88,,,,,,,10,1,1,1,9,,,,,,,;80;81,,,,,,,82,,,,,;120;211;81;113,,,9,,;177;156;178;66;180;233,3;110;24;96;96,15,,15,,15,,15;|4|1.1|72|72|72|72",
"73;129,,,,7,,,;76;19,9,7,9,104,7;77;51,9,104,7;117;138;118;133;120;194,11;81;40,15,33,15,49,15,33,15;179;42;180;236;112;6,,,,237,,,;|4|1.1|72|72|72|72",
"77;59;0;176,1,1,1;121;91;75;107;76;27,1,80,1;73;155;|4|4.1|72|72|0|74"
}
foreach(cz,cj)
da=0
db=false
dc=false
cv={1,4,5,17,27,25,20,2,16}
dd={
"116;50,51;81;17,18,19,20",
"116;49;119;4;81;1,18,35,52",
"73;48,53;116;49,52;119;2,3;81;17,20,34,35",
"75;48,53;81;18,19;177;51",
"75;48,53;81;18,19;178;51",
"73;48,53;80;34,35;116;49,50,51,52;81;1,2,3,4",
"76;0,5;77;16,21;81;1,2,3,4;180;51",
"75;32,37;76;48,53;81;49,50,51,52;179;34",
"76;0,5;77;16,21;81;1,2,3,4;179;19",
"116;32,37;75;48,53;73;2,3;119;18,19;81;34,35,50,51"
}
de={
"117;19,35,51;118;16,32,48;81;33,34,49,50",
"116;48,49,50,51;81;16,17,18,19",
"116;48,49,50,51;81;1,2,16,19",
"116;48,49,50,51;81;1,2,17,18",
"81;1,2,17,18;177;50",
"81;1,2,17,18;180;48,50",
"81;49,50;73;1,2;119;17,18;177;50",
"73;2,3;116;48;119;18;81;19,35,51",
"116;48,49,50,51;81;1,2,17,18;179;34",
"181;50;180;48"
}
end
function _update()
if a==7 then
a=1
bf=128
p=3
be=256
df()
b.c=315
b.d=94
v=43
u=1
music(38)
end
if a<2 then
dg()
dh()
if a==1 then
if bg!=0 then
if be!=256 then
local di=dj(be-256)
be+=di*(bg-time())*20
if(be>256 or di==1)
and(be<256 or di==-1) then
be=256
bg=0
bi=time()+1
end
end
if bf!=128 then
local dk=dj(bf-128)
bf+=dk*(bg-time())*20
if(bf>128 or dk==1)
and(bf<128 or dk==-1) then
bf=128
bg=0
bi=time()+1
end
end
if time()-bg>0.2 then
local cl=ci[cw-1]
if bh then
dl(cu,71,true)
else
dl(cu,76,true)
end
end
end
camera(max(0,be),max(0,bf))
else
be=b.c-64
if a==-1 then
if b.c>816 or bs>1 then
v=100
be=min(be+cu,792)
cu+=1
bp=max(0,cu-80)
if cu==60 then
music(48)
end
end
end
if bi<20 then
camera(max(0-a*256,be),0-a*128)
else
camera(924,min(128,b.d-67))
end
if b.c>550 and bc==""then
dm("spikes?")
end
if b.c>888 and bc=="spikes?"then
dm("dungeon!")
end
if b.c>976 and mget(0,2)==68 then
mset(121,7,68)
mset(125,7,68)
mset(121,9,68)
mset(125,9,68)
bi=1
mset(0,2,0)
end
if bi>0 and a==0 then
bi+=1
end
if bi==20 then
mset(122,10,0)
mset(123,10,0)
mset(124,10,0)
sfx(44)
end
if bi==24 then
sfx(48)
end
if bi==120 then
a=2
b.d=70
bi=0
music(18)
camera(0,0)
end
end
poke(0x5f00+92,255)
else
if a!=7 then camera(0,0) end
if btnp()!=0 then
if a==2 then
a=7
for dn=1,16 do
for dp=1,16 do
mset(dn,dp,0)
mset(dn+32,dp-1,0)
end
end
elseif a<6 then
a+=1
if a==4 then
sfx(25,3,0,3)
elseif a==5 then
sfx(25,3,0,3)
elseif a==6 then
bo=rnd(3)+cd*3
music(-1)
sfx(26,3,8,8)
o=time()+2
end
end
end
end
if a==6 and o<time() then
bu=0
a=7
cu=0
cw=1
q=3
cc=0
r=0
l=0
end
end
function _draw()
cls()
if a<2 then
if bp<80 then
map(0,0,0,0,128,32)
local dq=6*time()%6
if dq<3 then
pal(9,10)
pal(10,9)
pal(1,8)
pal(2,0)
else
pal(1,0)
pal(2,8)
end
if dq<2 then
pal(12,6)
pal(6,7)
pal(7,12)
elseif dq<4 then
pal(12,7)
pal(6,12)
pal(7,6)
end
map(0,0,0,0,128,32,0x10)
pal()
else
if btnp()!=0 then
br*=-1
bs+=1
bn=0
if bs==2 or bs==4 or bs==6 or bs==14 or bs==27 then
br*=-1
end
if bs==7 or bs==10 or bs==13 or bs==15 or bs==17 or bs==23 or bs==25 or bs==29 or bs==30 or bs==32 then
bn=-50
end
if bs==17 then
music(-1)
end
if bs==23 or bs==26 or bs==28 then
br*=2
elseif bs==25 or bs==27 or bs==29 then
br/=2
end
end
if bs<=31 then bn+=1
elseif bs==32 then
u=-80
sfx(0,3,0,6)
bs+=1
elseif u==-1 then
a=10
music(50)
end
if bn>0 then
rect(800,155,908,210,7)
print(sub(bm[bs],1,bn/2),810,165,7)
if abs(br)==1 then
line(854-br*20,210,854-br*32,228)
line(854-br*28,210,854-br*32,228)
line(854-br*21,210,854-br*27,210,0)
else
line(858,155,854,148)
line(850,155,854,148)
line(857,155,851,155,0)
end
end
end
if a==0 then
print("arrow keys to move",30,20,1)
print("z to jump",50,30,1)
end
if a==-1 then
print("princess storage\n\nroom ahead",272,178,1)
end
if a==1 and bi>time() then
rectfill(304,190,336,198,0)
print("level "..cw-2,306,192,1)
end
map(0,0,256,256,16,16)
map(0,0,128,128,16,16)
dr()
if cb>0 then
if w==3 then pal(8,9) end
sspr(24,72,16,16,bz,ca)
cb-=1
pal()
end
ds()
dt()
du()
local c=b.c
local d=b.d
if s%6<4 then
dv(c,d,f,e,5,g)
end
if bd!=0 then
local dw=#bc*4-1
local dx=c-flr(dw/2)+2
sspr(117,126,119,127,c+2,d-6)
rectfill(dx,d-13,dx+dw+1,d-7,7)
rectfill(dx-1,d-12,dx+dw+2,d-8)
print(bc,dx+1,d-12,1)
bd-=1
end
if b.d>156 then
z=min(z+2,129)
else
z=max(z-2,116)
end
if a>0 then
rectfill(264,127,375,10+z,0)
for dn=0,13 do
spr(14,264+dn*8,7+z)
end
sspr(51,16,2,11,299,z-1)
sspr(51,16,2,11,331,z-1)
spr(81,272,1+z)
print(l,285,2+z,7)
spr(188+w,303,z)
spr(188+x,311,z)
spr(188+y,321,z)
if p<1 then pal(8,2) end
spr(32,338,z)
if p<2 then pal(8,2) end
spr(32,346,z)
if p<3 then pal(8,2) end
spr(32,354,z)
if p>3 then pal(8,10)
elseif q==4 then pal(8,9)
else pal(8,0) end
spr(32,362,z)
pal()
end
sspr((bs>16 and 32) or 8,104,8,16,900+max(bp/-2,-16),232)
if bs>19 then bq+=1 end
if a!=0 then
spr(128,851,120+min(bq/15,18))
end
elseif a==2 then
print("dungeon!",50,21,5)
print("dungeon!",49,20,7)
print(" made by\ndeklaswas",47,104,3)
print("press anything to start",20,64,6)
elseif a==3 then
print("you died!",47,39,5)
print("you died!",46,38,7)
if time()-o>1.4 then
print(bc,65-2*(#bc),65,7)
end
if time()-o>2.6 then
spr(cw,60,84)
end
if time()-o>4.2 then
print("press anything to continue",13,110,6)
end
elseif a==4 then
print("coins this run:"..l,32,24)
print("high score:"..m,40,44)
print("press anything to continue",13,110,6)
elseif a==5 then
print("pro tip:",49,34,5)
print("pro tip:",48,33,7)
print(bl[flr(bo)+1].."!",0,64)
elseif a==6 then
print("attempt "..by,45,62,7)
end
if a==10 then
camera(792,128)
print("thank you for playing!",810,174,7)
dv(851,220,1,0,5,false)
end
end
function df(dir)
local dy=cv[cw]
if cw==25 then
dy=51
end
if cx==16 then
cc+=1
cd=max(cc,cd)
music(41-cc*10)
end
bh=dir
for dn=0,16 do
for dp=0,16 do
mset(dn,dp,mget(dn+32,dp+16))
mset(dn+32,dp+16,0)
end
end
cf={}
ch={}
local cl=ci[dy]
for dn=0,15 do
dz(32,16+dn,cl.cn)
dz(47,31-dn,cl.co)
dz(32+dn,16,cl.cm)
dz(47-dn,31,cl.cp)
end
if cw!=1 then
if dir then
bf+=128
else
be-=128
u=8
end
bg=time()
cu=ci[cx].cs
if dy!=16 and
cw!=25 then
for dn=0,10 do
dz(33+rnd(14),17+rnd(14),79)
end
end
end
cx=cv[cw]
local cq=split(cl["schem"],";")
local ea=#cq/2
for dn=1,ea do
local eb=cq[dn*2-1]
local ec=0
local ed=split(cq[dn*2])
for dp=1,#ed do
if ed[dp]==""then
ed[dp]=1
elseif ed[dp]==1 then
ed[dp]=16
end
ed[dp]+=ec
ec=ed[dp]
local ee=32+ed[dp]%16
local ef=flr(ed[dp]/16)+16
if eb==110 or eb==94 then
local eg=split(rnd(dd),";")
if eb==94 then eg=split(rnd(de),";") end
for eh=1,#eg/2 do
local ei=split(eg[eh*2])
for ej=1,#ei do
dz(ei[ej]%16+ee,flr(ei[ej]/16)+ef,eg[eh*2-1])
end
end
else
dz(ee,ef,eb)
end
end
end
dl(cu,0,true)
dl(cl.cs,0)
if cw==25 then
dz(48,31,74)
dz(48,16,72)
a=-1
bi=0
music(-1)
cu=0
bm[4]="but it only took me\n"..by.." tries!"
if x==4 then
bm[2]="...are you in your\nunderwear?"
end
end
cw+=1
if cx==1 then
if rnd()>0.9 then
ek(316,247,4,0,0)
cf[1].el=0
end
end
end
function dz(c,d,eb)
if eb>70 and eb<80 then
eb+=cc*16
end
if eb==81 then bx+=1 end
if eb==89 and rnd(3)>2 then
eb=88
end
if eb==105 then eb=104
elseif eb==104 then eb=105 end
if eb>175 and eb<186 then
ek(c*8,d*8,eb-176,0,dj(b.c-c*8))
else
mset(c,d,eb)
end
end
function dl(cr,em,en)
local dx=0
if cr==4.1
or cr==3
or cr==1.1
then dx=15 end
if en then
if dx==0 then dx=15 else dx=0 end
end
local eo=0
if cr==5.7
or cr==3 then eo=5 end
if cr==4.1 then eo=10 end
if cr==4 or cr==5.7
then
for dn=0,3 do
dz(33+dn+eo,16+dx,em)
end
elseif cr!=0 then
for dn=0,3 do
dz(32+dx,17+dn+eo,em)
end
end
end
function dj(ep)
return ep>0 and 1 or(ep==0 and 0 or-1)
end
function eq(dir,er)
local es=b.c
local et=b.d
if dir then
es+=f
else
et+=e
end
local eu=5+es
local ev=12+et
local ew=et+6
if er==1 or er==3 then
if fget(ex(es,et),er)
or fget(ex(es,ev),er)
or fget(ex(es,ew),er)
or fget(ex(eu,et),er)
or fget(ex(eu,ew),er)
or fget(ex(eu,ev),er) then
return true
else return false
end
elseif er==81 then
local ey=5
for dn=0,3 do
local ez=et+ey*dn
if ex(es,ez)==er then
mset(es/8,ez/8,0)
l+=1
n+=1
return true
end
if ex(eu,ez)==er then
mset(eu/8,ez/8,0)
l+=1
n+=1
return true
end
end
elseif er==0 then
if fget(ex(es,ev),er)
or fget(ex(eu,ev),er) then
return true
else return false
end
end
end
function fa(c,d,fb,fc,dir)
local fd={
c=c,
d=d,
fe=fb,
fc=fc,
el=2.72,
ff=dir
}
add(cg,fd)
if fc==3 then
if dir==1 then fd.el=-2.35
else fd.el=-2.35 end
end
end
function du()
for fd in all(cg) do
local fg=b.c
local fh=b.d
if fd.fc<2 then
fd.el+=1
local fi=(fd.fc*10)-9
fd.c=fg+3+fd.ff*7*fi
fd.d=fh+8
if fi>1 then
spr(13+fi,fd.c-4,fd.d-4,1,1,fd.ff<0)
else
spr(12+fi,fd.c-4,fd.d-4,1,1,fd.ff<0)
end
if fd.el>10 then
del(cg,fd)
end
elseif fd.fc==2 then
local c=fd.c
fd.c+=5*fd.ff
spr(46,c-4,fd.d-3,1,1,fd.ff<0)
if fget(ex(fd.c,fd.d),1) then
del(cg,fd)
sfx(20,3,4,1)
end
elseif fd.fc>3 then
if fd.fc==5 then
local fg=b.c
local fh=b.d
end
if fd.c<fg then fj=-1 end
fd.el+=0.03
fd.d=fh+7+(sin(fd.el)*25)
fd.c=fg+3+(fd.ff*cos(fd.el)*25)
spr(201,fd.c-4,fd.d-4)
line(fg+3,fh+7,fd.c,fd.d,13)
if fd.el>3.72 then
del(cg,fd)
end
elseif fd.fc==3 then
fd.el+=0.05
fd.d=fh+(sin(fd.el)*20)-14
fd.c=fg+(fd.ff*cos(fd.el)*20)+(10*fd.ff)+4
local fk=sin(4*fd.el)<0
local fl=cos(4*fd.el)<0
spr(200,fd.c-4,fd.d-4,1,1,fk,fl)
if fd.el>-1.40 then
del(cg,fd)
end
end
end
end
function fm(fn)
p-=1
if p<1 and a==1 then
by+=1
bx=0
r=1
cw=fn+176
if fn==3 then
bc="a bat killed you"
elseif fn==2 then
bc="a wizard killed you"
elseif fn==1 then
bc="a skeleton killed you"
elseif fn==0 then
bc="a skull killed you"
elseif fn==4 then
bc="a slime killed you"
elseif fn==-1 then
bc="some spikes killed you"
elseif fn==-2 then
bc="a mimic killed you!"
elseif fn==6 then
bc="you can not jump on spiders!"
end
bd=0
music(-1)
sfx(48,2)
v=3000
o=time()+3
return true
else
sfx(57,3,0,6)
if a!=0 then dm("pain") end
if x==6 and p==1 then
x=4
q=3
dm("armor")
sfx(48,3)
bd=70
fo(b.c,b.d,-1,{0},0,dj(f))
else
sfx(57,3,0,16)
end
s=30
return false end
end
function ex(c,d)
return mget(c/8,d/8)
end
function fp()
cy={1}
for dp=0,2 do
for dn=0,6 do
repeat
db=true
dc=true
da=flr(rnd(30))+1+dp*10
if dn==6 then
da=flr(rnd(30+dp*10))+1
end
for fq in all(cy) do
if da==fq then db=false end
end
local fr=#cv
for eh=0,7 do
if da==cv[fr-eh] then dc=false end
end
until(ci[da].cr==flr(ci[cv[fr]].cs)
and(dn!=6 or ci[da].cs==4.1)
and(dc
or dn==6)
and db)
add(cv,da)
if ci[da].ct==true then
add(cy,da)
end
end
if dp==2 then
add(cv,51) else
add(cv,16) end
end
end
function fs(ft)
if ft then
p=min(p+1,q)
sfx(26,3,0,8)
else
l+=10
n+=10
sfx(27,3,0,4)
end
end
function dg()
local c=b.c
local d=b.d
if r==0 then f*=0.2 else
if(a==1 and o<time()) or a==10 then
o=time()
b.c=7
b.d=105
cv={1}
a=3
camera(0,0)
m=max(n,m)
ce={}
cf={}
ch={}
sfx(-1,3)
sfx(-1,2)
music(20)
w=0
x=5
y=9
end
end
if w==2 then ba=2 else ba=1.5 end
e=min(e+0.3,bb)
h=max(h-1,0)
j=fget(ex(c,d+13),0)
or fget(ex(c+5,d+13),0)
if j then h=0 end
if j and w==1 then k=true end
local fu=btn(⬅️) and u<1
local fv=btn(➡️) and u>-1
local fw=btnp(🅾️) or btnp(⬆️)
local fx=btn(⬇️)
local fy=btn(❎)
if v!=0 then
v-=1
if v==0 and a==1 and cw==2 then
fp()
music(0)
bu=128
end
fw=false
fx=false
fu=false
fv=false
fy=false
end
if s!=0 and r==0 then
s-=1
end
local fz=y-9
if fy and o<time() and fz!=0 then
local ga=1
if g then ga=-1 end
local gb=c+3+ga*10
if fz==1 then
fa(gb+ga*8,d+6,"en",1.1,ga)
fa(gb+ga*16,d+6,"en",1.2,ga)
o=time()+0.7
elseif fz==2 then
sfx(27,3,29,1)
o=time()+1
elseif fz==4 then
sfx(27,3,14,10)
o=time()+1.1
elseif fz==3 then
sfx(27,3,6,7)
o=time()+1
end
fa(gb,d+6,"en",fz,ga)
end
if h!=0 then
ba*=1.1
end
if fu or u<0 then f-=ba
g=true
end
if fv or u>0 then f+=ba
g=false
end
if h!=0 then
ba/=1.1
end
if u!=0 then
u-=dj(u)
end
local gc=1
if g then gc=0 end
if j and fw then
gd(c+3,d+10,10,5,{7,6,5})
e=-4
j=false
sfx(25,3,0,3)
elseif
not j and
fget(ex(c+2.5+dj(f)*3.5,d+6),1)
and(btn(⬅️) or btn(➡️))
then
bb=1
if x==8 and not fx then bb=0 end
g=not g
if fw then
ge(c+gc*7,d+6,10,3,{7,6,5})
sfx(25,3,0,3)
e=-3
f=ba-gc*ba*2
u=15*dj(f)
h=15
end
elseif k and fw then
gd(c+3,d+10,10,5,{7,12,1})
sfx(25,3,0,5)
e=-4
k=false
else bb=4
end
if r==0 then
if eq(true,81) then
sfx(27,3,0,2)
end
local gf=ex(c+2+gc,d+6)
if fget(gf,3) and s==0 then
fm(-1)
if gf==116 then
e=-4.5
elseif gf==117 or gf==118 then
e=-2
v=10
u=-10*dj(f)
elseif gf==119 then
e=2
end
end
if eq(true,1) then
local dj=dj(f)
f=dj
while(not eq(true,1))
do b.c+=dj
end
f=0
else
b.c+=f
end
local gg=0
if dj(e)==-1
then gg=1 end
if fx or fget(ex(c+2.5,d+12),0)
then gg=1 end
if eq(false,gg) then
local dj=dj(e)
e=dj
while(not eq(false,gg))
do b.d+=dj
end
e=0
end
b.d+=e
else
b.c+=f
b.d+=e
end
c=b.c
d=b.d
if ex(c+3,d)==96 then
gh(b.c+2.5,b.d+6,5,20,{12})
b.c=634-c-dj(320-c)
gh(b.c+2.5,b.d+6,5,20,{12})
elseif ex(c,d+6)==112 then
gh(b.c+2.5,b.d+6,5,20,{12})
b.d=372-d-dj(192-d)
gh(b.c+2.5,b.d+6,5,20,{12})
end
if a==1 and r==0 then
if c+3>bv then
b.c=bt+c-bv-1
df()
sfx(0,3,0,6)
end
if d+3<bu then
e=-4
b.d=bw-bu+d+1
df(true)
sfx(0,3,0,6)
end
end
end
function dv(c,d,gi,gj,gk,g)
local gl=gk==9
local en=0
if g then en=1 end
i=0
local gm=49
local gn=18
local go=6
local gp=22
if abs(gi)>0.5 then
local t=flr(time()*10)%4
gp=23+t/2
go=7+t/2
gm=1+t*16
gn=2+t*16
else
end
if abs(gj)>1 then
if gj<0 then
gn=50
gm=1
elseif gj>0 then
gn=2
gm=17
gp=25
end
elseif gl or not j then
gn=2
gm=1
end
if w==1 then pal(8,12)
elseif w==2 then pal(8,11)
elseif w==3 then pal(8,9) end
if x==6 or gl then pal(6,10) pal(5,9)
elseif x==7 then pal(6,13) pal(5,1)
elseif x==8 then pal(6,14) pal(5,2) end
if gk==21 then
pal(6,1)
pal(5,13)
pal(8,2)
elseif not gl then
spr(go,c-1,d-8,1,1,g)
if x==4 then
gn+=2
gm+=2
end
end
spr(gk,c-1,d-3,1,1,g)
line(c+2,d+5,c+3,d+5,8)
if x!=4 or gk==21 then
spr(gp,c-3+en*4,d+6,1,1,g)
end
spr(gm,c-5+en*8,d+5,1,1,g)
spr(gn,c+3-en*8,d+5,1,1,g)
pal()
end
function dm(fc)
local bs={fc}
if fc=="pain"then
bs={
"ouch!",
"hey!",
"oops!",
"ow!",
"crap!"
}
elseif fc=="stomp"then
bs={
"bop!",
"bam!",
"pow!",
"hah!",
"stomp!",
"nice!"
}
elseif fc=="melee"then
bs={
"engarde!",
"attack!",
"pow!",
"bam!"
}
elseif fc=="ranged"then
bs={
"bullseye!",
"nice!",
"bam!"
}
elseif fc=="armor"then
bs={
"my suit!",
"my armor!",
"my underwear!"
}
elseif fc=="fire"then
bs={
"fire!",
"it burns!",
"hot!"
}
elseif fc=="treasure"then
bs={
"treasure!",
"sweet!",
"loot!"
}
end
bc=rnd(bs)
bd=40
end
function ek(c,d,fb,gq,ff)
local gr=false
local p=1
local gs={
c=c,
d=d+8,
gt=fb,
gu=gq,
gi=1.2*ff,
ff=ff,
gj=0,
el=time(),
gv=0,
gb=8,
gw=-12,
gx=0,
gy=false,
gz={-1},
ha=time()+0.6
}
if fb==0 then
gs.gi=0
p=2
if cc==2 and gq!=1 then
p=5
gs.gv=16
gr=true
gs.gz={0,11}
elseif cw>7 then
p=3
gs.gq=1
gs.gv=16
gs.gy=true
end
elseif fb==1 then
gs.gw=-16
if cc==2 then
p=2
gs.gy=true
elseif cc==1 and gq!=1 then
gr=true
end
elseif fb==2 then
p=2
gs.gi*=0.5
gs.gw=-16
gs.el+=1.3
gr={{14},{1,13},{10}}
if cc==2 then
p=4
gs.gy=true
gr={{5},{9,8},{10}}
elseif cc==1 then
p=3
gs.gz={5,1}
gs.gq=1
gs.gj=16
gr={{5},{5,6},{1}}
end
elseif fb==3 then
gs.gi*=1.5
if cc>0 then
gs.gv=16
gs.gy=true
end
elseif fb==4 then
gs.gz={11,3}
gs.gi*=0.5
gs.d-=16
gs.el-=rnd()
if rnd()>0.9 then
gs.gz={10,9}
elseif rnd()>0.9 then
gs.gz={14,8}
end
if cc==2 then
p=2
gs.gy=true
gs.gz={9,8}
elseif cc==1 and gq==0 then
gs.gx={13,1}
gs.gz={13,1}
gs.gu=3
elseif gq==2 then
gs.gx={10,6}
gs.gz={10,6}
gs.gv=20
gs.p=3
end
elseif fb==5 then
gs.gi=0
if cc==1 and(rnd()>0.5 or cv[cw]==38) then
gr=true
end
elseif fb==6 then
gs.gi=0
gr=true
p=4
if cc==2 then
gs.gz={8,5}
end
elseif fb==9 then
gs.gi=0
gs.gw=0
music(13)
local hb={
{1,2},
{6,7},
{10,11,12}
}
if cc==1 then
hb={
{1,2,3},
{6,7,8},
{10,11,12,13}
}
end
local hc=rnd({1,2,3})
gs.c=rnd(hb[hc])
del(hb,hb[hc])
gs.d=rnd(rnd(hb))
gs.gu=flr(bx*0.4)
bx=0
end
gs.gr=gr
gs.p=p
add(cf,gs)
end
function dh()
for gs in all(cf) do
local c=gs.c
local d=gs.d
local gi=gs.gi
local gt=gs.gt
local fg=b.c
local fh=b.d
if gt==1 or gt==2 or gt==3 or gt==4 then
local dx=c if gi>0 then dx+=6 end
if(not(fget(ex(dx+gi,d),0)) and gt!=3 and gt!=4)
or fget(ex(dx+gi,d-1),1) then
gi*=-1
gs.ff*=-1
end
end
if gs.gy then
if gt==0 then
hd(c,d-3,6,10,{9,8,8,5})
end
if gt==1 then
hd(c+gi*2,d-6,6,7,{9,8,8,5})
hd(c+gi*2,d,6,4,{9,8,8,5})
end
if gt==3 then
hd(c+gi,d,6,10,{8,8,5})
end
if gt==4 or gt==2 then
hd(c+gi,d+24+gs.gw*2,6,1,{5})
end
end
if gs.gr and gt==0 then
hd(c,d-3,6,10,{11,13,13,2})
end
if gt==0 then
c-=(c-fg)/50
d-=(d-fh-6)/50
elseif gt==2 then
if gs.gx==1 then
he(3+c+gs.ff*12,d-6,gs.gr)
else
gs.gv=4*time()%2
end
if(fh<d or gs.gx!=0) and time()>gs.el then
if gs.gx==0 then
gi=0
gs.gx=1
gs.gv=16
gs.el=time()+1.4
sfx(16,3,2,7)
if gs.gy then sfx(21,3,0,7) end
if fg<c then
gs.ff=-1 else gs.ff=1
end
else
if gs.gq==1 then
sfx(26,3,8,8)
ek(3+c+gs.ff*12,d-8,1,flr(rnd(3)),gs.ff)
else
sfx(22,3,0,4)
ek(3+c+gs.ff*12,d-13,3,0,gs.ff)
end
gs.gv=0
gs.el=time()+5
gi=gs.ff*0.6
gs.gx=0
end
end
elseif gt==4 then
if time()-gs.el>2 then
gs.gj=-3
gs.el=time()
end
d+=gs.gj
gs.gj+=0.3
if fget(ex(c+4,d+gs.gj),1) then
while(fget(ex(c+4,d),1))
do d-=dj(gs.gj)
end
if gs.gj>1 then
hf(c+3,d,10,5,gs.gz)
end
gs.gj=0
end
d+=gs.gj
if gs.gj==0 then
gi=0
else
gi=1.2*gs.ff
end
elseif gt==5 and gs.gr then
if abs(fg-c)<26 then
ek(c,d+2,4,2,gs.ff)
del(cf,gs)
sfx(0,3,8,8)
gd(c+3,d,10,5,{7,6,5})
end
elseif gt==6 then
if fget(ex(c,d+gs.ff),1) then
gs.ff*=-1
end
d+=gs.ff
elseif gt==9 then
if gs.gx==0 and fh<192 then
gs.gx=1
dm("greetings!")
end
z=131
if fh<158 then
if fg<325 then
gs.gv=c
gs.gx=gs.gu
elseif fg<349 then
gs.gv=d
gs.gx=gs.gu
else
gs.gv=0
gs.gx=10
end
if btnp(❎) and gs.gv!=-1 then
if l>=gs.gx then
sfx(27,3,0,4)
l-=gs.gx
if gs.gv==c then
hg(c)
c=-1
elseif gs.gv==d then
hg(d)
d=-1
elseif gs.gv==0 then
fs(true)
end
else
sfx(25,3,16,8)
end
end
end
end
if cx>40 then
if c>bv then
c=bt
elseif c<bt then
c=bv
end
end
gs.c=c+gi
gs.d=d
for fd in all(cg) do
if c<fd.c and fd.c<c+gs.gb and
d+gs.gw<fd.d and fd.d<d
and gt!=5
and gs.ha<time() then
local hh=2
sfx(22,3,16,1)
if fd.fc==2 then
del(cg,fd)
hh=1
elseif fd.fc==4 then
hh=3
end
gs.ha=time()+0.6
hi(1,gs,hh,fd.c,fd.d)
end
end
if(c<fg and fg<c+gs.gb or
c<fg+5 and fg+5<c+gs.gb) and
d+gs.gw<fh+4 and fh+4<d
and r==0 then
if e>0
and(gs.gr!=true or x==8)
and(gs.gy!=true or x==7) then
e=-4
b.d-=4
sfx(25,3,0,8)
if gs.gz[1]==10 then
fs(false)
elseif gs.gz[1]==14 then
fs(true)
end
hi(0,gs,10,fg+2.5,fh+8)
if rnd()<0.4 then
dm("stomp")
end
if gt==5 then
dm("treasure")
fs(p!=q)
end
else if s==0 and gt!=5 then
fm((gs.gv==20 and-2) or gt)
if p==0 then
f=-dj(f)*2
e=-4
else
if gs.gy then
dm("fire")
end
v=5
if dj(f)==-1 then
hj=10 elseif dj(f)==1 then
hk=10 end
e=-2
s=30
if gt==1 or gt==2
and dj(f)!=dj(gi) then
gi*=-1
gs.ff*=-1
end
end
end
end
end
gs.gi=gi
end
end
function ds()
for gs in all(cf) do
local c=gs.c
local d=gs.d-8
local gv=gs.gv
if gs.gy then
hl()
end
if gs.gz[1]!=-1 then
pal(1,gs.gz[1])
pal(2,gs.gz[2])
pal(10,gs.gz[1])
pal(9,gs.gz[2])
pal(11,gs.gz[1])
pal(3,gs.gz[2])
end
local hm=gs.ff==-1
local dir=0
if gs.ff==-1 then dir=1 end
if gs.gt==0 then
spr(128+gv,c,d,1,1,c<b.c)
elseif gs.gt==1 then
spr(130+9*time()%4+gv,c,d,1,1,hm)
spr(129,c,d-8,1,1,hm)
if gs.gr then
sspr(112,64,9,8,c-dir,d-12)
end
elseif gs.gt==2 then
spr(145+gv,c,d,1,1,hm)
sspr(97-gs.gj,80,11,7,c-1-dir,d-8,11,7,hm)
if gv==16 then
pset(c-1,d,2)
pset(c+8,d)
end
elseif gs.gt==3 then
spr(149+5*time()%2+gv,c,d,1,1,hm)
elseif gs.gt==4 then
spr(136+5*(time()-gs.el)%2+gv,c,d+1)
elseif gs.gt==5 then
spr(156,c,d)
elseif gs.gt==6 then
line(c+4,136,c+4,d,5)
sspr(64,80,15,8,c-2,d)
elseif gs.gt==9 then
if bd>0 and bd<27 then
sspr(109,118,19,10,265,179)
end
spr(188+c,312,151)
spr(196+d,336,151)
spr(10,360,151)
if b.d<158 and gv!=-1 then
print("  press x\nto purchase",317,185,1)
rectfill(264,128+88,256+119,128+119,0)
local hn=bj[gv+1]
print(hn,321-2*(#hn+8),220,7)
print(gs.gx.." coins",319+2*(#hn),220,10)
hn=bk[gv+1]
print(hn,266,231,6)
end
end
pal()
end
end
function fo(c,d,fb,gq,ho,ff,gv)
local hp={
c=c,
d=d,
gt=fb,
gu=gq,
ho=ho,
gi=1.2,
gj=0,
ff=ff,
el=time(),
gv=gv,
hq=c,
hr=d
}
if fb==-1 then hp.gj=-3 end
add(ch,hp)
end
function dt()
for hp in all(ch) do
if time()-hp.el>1.2 then
del(ch,hp)
end
local c=hp.c
local d=hp.d
if hp.gu==-1 then
hl()
elseif hp.gu[1]!=-1 then
pal(1,hp.gu[1])
pal(2,hp.gu[2])
pal(11,hp.gu[1])
pal(3,hp.gu[2])
end
local hm=hp.ff==-1
if hp.gt==-1 then
hp.c+=0.6*hp.ff
hp.d+=hp.gj
hp.gj=min(hp.gj+0.2,5)
dv(hp.c,hp.d,0,-1,9,hm)
elseif hp.gt==1 then
if hp.ho==0 then
spr(130,c,d-8,1,1,hm)
spr(129,c,d-13,1,1,hm)
else
hp.c-=0.6*hp.ff
hp.hq+=0.6*hp.ff
hp.d+=hp.gj
hp.hr+=hp.gj
hp.gj=min(hp.gj+0.3,5)
spr(129,hp.hq,hp.hr-16)
spr(130,c,d-8)
end
elseif hp.gt==3 then
hp.c+=0.6*hp.ff
hp.d+=hp.gj
hp.gj=min(hp.gj+0.3,5)
local hs=((12*time())%2)>1
spr(150+hp.gv,c,d-8,1,1,hs,0)
elseif hp.gt==2 then
local dir=0
if hp.ff==-1 then dir=1 end
spr(161,c,d-8,1,1,hm)
pset(c-1,d-6,2)
pset(c+8,d-6,2)
sspr(97,81,11,5,c-2+dir,d-13,11,5,hm)
elseif hp.gt==4 then
spr(138,c,d-6)
end
pal()
end
end
function hi(fn,gs,ht,es,et)
if w==3 then ht*=2 end
gs.p-=ht
bz=es-8
ca=et-8
cb=3
if gs.p<1 then
del(cf,gs)
local gq=gs.gz
if gs.gy then gq=-1 end
fo(gs.c,gs.d,gs.gt,gq,fn,gs.ff,gs.gv)
if gs.gt==4 then
hf(gs.c+3,gs.d,10,15,gs.gz)
if gs.gu==3 then
ek(gs.c,gs.d+8,4,1,gs.ff)
ek(gs.c,gs.d+8,4,1,-gs.ff)
end
elseif gs.gt==0 then
gh(gs.c+3,gs.d-8,5,10,{7})
end
end
end
function hl()
pal(7,10)
pal(3,8)
pal(11,9)
pal(5,9)
pal(1,8)
pal(2,9)
end
function hg(hu)
if hu<4 then
w=hu
elseif hu<9 then
x=hu
if hu==6 then p+=1
q=4 else
p=min(p,3)
q=3
end
else
y=hu
end
end
function hv(c,d,hw,hx,hy,hz,ia,ib,ic)
local id={
c=c,
d=d,
t=0,
hw=hw,
hx=hx,
hy=hy,
hz=hz,
ia=ia,
ib=ib,
ic=ic
}
if#ce<1000 then
add(ce,id)
end
end
function dr()
for id in all(ce) do
local t=id.t
local ie=0
id.t+=1
if t>=id.hw then del(ce,id) end
if t/id.hw<1/#id.ic then
ie=id.ic[1]
elseif t/id.hw<2/#id.ic then
ie=id.ic[2]
else ie=id.ic[3]
end
id.hy+=id.hz
id.ib+=id.ia
id.c+=id.hx
id.d+=id.hy
if id.ib<1 then
pset(id.c,id.d,ie)
else
circfill(id.c,id.d,id.ib,ie)
end
end
end
function gd(c,d,gb,ea,ic)
for dn=0,ea do
hv(
c+rnd(gb)-gb/2,
(d+2)-rnd(2),
10,
(rnd(4)-2)*1.1,
0,
0,
-0.2,
3,
ic
)
end
end
function ge(c,d,gb,ea,ic)
for dn=0,ea do
hv(
c-rnd(2),
d+rnd(gb)-gb/2,
10,
0,
(rnd(4)-2)*0.9,
0,
-0.2,
3,
ic
)
end
end
function hf(c,d,gb,ea,ic)
for dn=0,ea do
hv(
c+rnd(gb)-gb/2,
d,
12+flr(rnd(4)),
(rnd(4)-2)*0.9,
0,
0,
0,
0,
ic
)
end
end
function gh(c,d,gb,ea,ic)
for dn=0,ea do
hv(
c+rnd(gb)-gb/2,
d+rnd(gb)-gb/2,
12+flr(rnd(4)),
(rnd(4)-2)*2,
(rnd(4)-2)*0.4,
0.1,
0,
0,
ic
)
end
end
function hd(c,d,gb,ea,ic)
for dn=0,ea do
hv(
c+rnd(gb)-gb/2+3.5,
d+rnd(gb)-gb/2-3,
13,
0,
-0.7,
0,
-0.2,
2,
ic
)
end
end
function he(c,d,ig)
ih(10,0,3+c,d,ig[1])
ih(10,3,3+c,d,ig[2])
ih(4,0.5,3+c,d,ig[3])
end
function ih(ii,ib,c,d,ic)
for dn=0,4 do
hv(
c+rnd(2)-1,
d+rnd(2)-1,
ii,
(rnd(4)-2)*0.4,
(rnd(4)-2)*0.7,
0,
-0.4,
ib,
ic
)
end
end
__gfx__
00000000000000000000000000000000000000000000000000000000000000000000000000000000000940000000000000000000000004000000000000000000
0000000000005566566500000000ffffffff00000000000000000000000000000000000000000000006766000000000000000000000044000000000000000000
007007000005066656605000000f0ffffff0f0000000000000000000000000000000000000000000000670000000000000000000000440000000000000000000
000770000006005555006000000f00ffff00f0000066660000000000000000000000000000000000007887000000000000000000499477777777777777777700
00077000000000666600000000000078780000000665555000000000000000000000000000000000007e86000000000000000000499466666666666666666666
0070070000000050005000000000008000700000065656560088800000888000808880000000000007e882600000000000000000000440000000000000000000
0000000000006600000600000000ff00000f00000665555008888000888880000888800000000000068222600000000000000000000044000000000000000000
0000000000006000000660000000f000000ff0000066660080080000000800000008000000000000006666000000000000000000000004000000000000000000
00067000000000000000000000000000000000000000000000088888000888880008888888888800000000000000000000000000000000000000000000000000
0006700000005566566500000000ffffffff00000000000000088888008888800088888000888888000000000000000000000000000000000000000000000000
000670000005066656605000000f0ffffff0f0000000000000088888008888000888880000000000000000000000000000000000000000000000000000000000
000670000006005555006000000f00ffff00f0000111111000088888088888008888800000000000000000000000000000000000000000000000000000000000
00067000000000666600000000000078780000000111111000088888888880000000000000000000000000000900000000000000000000000000000000000000
0006700000006650050000000000ff80070000000100000000088888000000000000000000000000000000099900000000000000000000000000000000000000
0006700000006000060000000000f0000f000000011101100000000000000000000000000000000000000999ffff000000000000000000000000000000000000
000670000000000006600000000000000ff00000001101000000000000000000000000000000000000009fffffff000000000000000000000000000000000000
0000000000000000000000000000000000000000000000000006700000000000000000000000000000009f1f11ff770000000000000000000000000000000000
0000000000005566566500000000ffffffff0000000000000006700000000000000000000000000000009ff1ff1f766000000000000000000000000000000000
000000000005066656605000000f0ffffff0f000000000000006700000000000000000000000000000009fffffff566000000000000000007600006000000000
088888880006005555006000000f00ffff00f000000000000006700000000000000000000000000000009f11ff1f776000000000000000000044446700000000
8888888000000066660000000000007878000000000000000006700000000000000000000000000000009f1f1f1f560000000000000000006700006000000000
00000000000006650050000000000ff800700000000000000006700000000000000000000000000000009fffffff560000000000000000000000000000000000
00000000000006000600000000000f000f000000000000000006700000000000000000000000000000000fffffff600000000000000000000000000000000000
000000000000000006600000000000000ff0000000000000000670000000000000000000000000000000000fffff000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000067000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000005566566500000000ffffffff00000000000000067000000000000000000000000000000000000000000000000000000000000000000000000000
000000000005066656605000000f0ffffff0f0000000000000067000000000000000000000000000000000000000000000000000000000000000000000000000
022222220006005555006000000f00ffff00f0000000000000067000000000000000000000000000000000000000000000000000000000000000000000000000
22222220000000666650000000000078787000000000000000067000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000000050006000000000008000f000000000000000067000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000006000660000000000f000ff00000000000000067000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000006600000000000000ff000000000000000000067000000000000000000000000000000000000000000000000000000000000000000000000000
00000000220220223b3bbb3b2222222000000000900000090000000000000000220220226dddddd220222202ddddd1110d0d10100d0d10100000000000100010
0000000011022011b030b030202020010000000049aaaa9400011000dddddddd11011011d22d2d2101110101000000000d0d10100d0d10100000000011111111
0000000000222200030bb003220000010000000090000009000210000000000000000000dd222221110010100d0d10100d0d10100d0d10100000000000010000
00000000201002020000b00020020101000000009000000900021000dddddddd20222202d2d22211001001010d0d10100d0d10100d0d10100000000001111110
000000001010020100000000220000010000000090000009000210001111111110111101d2221221000000000d0d10100d0d10100d0d10100000000001000000
000000000011120000000000200010110000000049aaaa94000210000000000000000000d2121211000000000d0d10100d0d10100d0d10100000000011111111
000000002201102200000000201001010000000090000009000210001111111122022022d2212121000000000d0d10100d0d1010000000000000000000000100
00000000110110110000000001111111000000009000000900000000000000001101101121111111000000000d0d10100d0d1010ddddd1110000000000111110
99099909000000003b330b0330b033b322222222000000000001100000000000360630637666666db0b3b30b66666ddd0606d0d00606d0d00000000000300030
40444044000aa000b100b3b00b3b001b22221222000000000002100066666666d30dd03d6dd6ddd1013b0101000000000606d0d00606d0d00700007033333333
0000000000aa9a00301100000000110322112212000000000002100000000000000000006ddd6dd1300030300606d0d00606d0d00606d0d00700007000030000
0000000000aa9a000b000010010000b0111221220000000000021000666666663063630666ddddd1001003010606d0d00606d0d00606d0d00700007003333330
0000000000aa9a003010000000000103111111110000000000021000ddddddddd0dd3d0d6ddd1dd1010000000606d0d00606d0d00606d0d00700007003000000
0000000000a99a00b30000000000003b01111110000000000000000000000000000000006ddddd11000001000606d0d00606d0d00606d0d00700007033333333
00000000000aa0003000000000000003000000000000000000222200dddddddd360330636dd1ddd1000100000606d0d00606d0d0000000000777777000000300
0000000000000000010000000000001000000000000000000111111000000000330dd03dd1111111000000000606d0d00606d0d066666ddd0000000000333330
076cc6700000000030100000000001032227770000000000000000000000000022022022ddddddd1606666062222211102021010020210100000000000200020
076cc67000010000b10010000001001b2227777700000000000880002222222288088088d1d1d110055505050000000002021010020210100777777022222222
076cc6700000010030100000000001032217766700000000008828000000000000000000dd111110550050500202101002021010020210100000000000020000
076cc670010000000b000000000000b01116666600000000008828002222222220222202d11d1010005005050202101002021010020210100000000002222220
076cc6700010030130100000000001031116666600000000008828001111111180888808dd111110000000000202101002021010020210100000000002000000
076cc67030003030b30000000000003b0000000000000000008228000000000000000000d1110100000000000202101002021010020210100000000022222222
076cc670013b010130010000000010030d0d101000000000000880001111111122022022d1011010000000000202101002021010000000000777777000000200
076cc670b0b3b30b01000000000000100d0d10100000000000000000000000008808808810000000000000000202101002021010222221110000000000222220
000000000022222201000000000000100000000776776ddddd666770dddddddd0101200000012000555555550000000002222210000000000000000099499949
77777777222212223000000000000003700000060000066dd6600000d6d6dd6d0218800000021000506006050000000022222111000000000777777099499949
6666666622112212b30000000000003b70070007000776dddd667000d660d6660028900002088010565665650000001020021002010000000700007044444444
cccccccc111221223010000000000103607607070000006dddd66700607066060009a00012188212506006050000011012221121011000000000000094994994
cccccccc111111110b000010010000b06066070600766dddd6000000707067060004500088898888506006050000111101121110111100000000000044444444
66666666000000003011000000001103666d066d000766dddd677000700070070004500008899880565665650000110001011010001100000700007099499499
777777770d0d1010b100b3b00b3b001bd6dd6d6d0000066dd66000006000000700045000089aa980506006050001110000000000001110000777777099499499
000000000d0d10103b330b0330b033b3dddddddd077666ddddd67767700000000000500000899800555555550001100000000000000110000000000044444444
07777700000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000
77777660000000000777577007775770077757700777577080088080000000000000000000000000000000000000000000000000000000000000700000000000
577556600000000050557505505575055055750550557505088888000000000000bbbb0000bbbb00000000000000000000000000000000000007760000000000
65667760007777707077770770777707707777077077770700888888000000000b3773b00b3733b0000000000000a00000000000000000000007660000000000
00055600075777570055750000557500005575000055757000888880000000000b3733b0b373333b00000000000a9a0000000000000000000000600000000000
0757560007775777007000707770070007770070007000700888880000000000b333333bb333333b00000000000a9a0000000000000000000066665000000000
0066600000777770770000707000070007000700007000770808808000000000b333333bb333333b30b3b0330000a00000000000000000000666666500000000
00000000007070707000007700000770000007700077000080080000000000000bbbbbb00bbbbbb00b0b33b00000000000000000000000005555555550000000
9aaaaa900022220000222200000000080000000010000001000e1e00000000000000000000000000000000000000000000000000000000000000000000000000
aaaaaaa9011211100112111000000008000000001100001101111100000000000000000000000000000000000000000000000000064446d00000000000000000
9aa99aa91012110110121101000000088000080011100011110101100000000000000000000000000000000000000000064446d066a9664d0000000000000000
a9aaaaa920112102201121028800008880008000011001101100001100000000000000000000000000000000000000006444644d00d000dd0000000000000000
99999a99001121000011210008880888880880000110010011000011000000000000007000000000000000000000000066a66ddd000d000d0000000000000000
9a9a9a9000112100011121100088888888880000001e1e001000000100000000000000700000000000000000000000006494644d6444644d0000000000000000
09aaa90001112110011121100008888888880000000110001000000100000000000007770000000000000000000000006444644d6444644d0000000000000000
00999000222222200222222000008888888888880001000000000000000000000000076600000000000000000000000066666ddd66666ddd0000000000000000
9aaaaa900022220000000000000088888888880090000009000a9a00000000000e000222000e0000000111100000000000011110000000000000000000000007
aaaaaaa911121111000000000000888888880000990000990999990000000000020022222002000000111111000000000011111100000000064446d070000006
9aa99aa90012110000000000000888888880000099900099990909900000000000e2222222e000000000111110000000001011111000000066a9664d70070007
a9aaaaa90011210000000000000888888808000009900990990000990000000000022222220000000001111111000000000011111100000000d000dd60760707
99999a990011210000000000008800888000800009900900990000990000000002e0222220e2000002222222222200000001122221000000000d000d60660706
9a9a9a9000112100000000000080000880000800009a9a009000000900000000e00e0eee0e00e000000077777000000002222000022200006444644d666d066d
09aaa900011121100000000008000008000000000009900090000009000000000020022200200000000070707000000000000a0a000000006444644dd6dd6d6d
009990002222222000000000000000080000000000090000000000000000000000e0022200e000000000000000000000000000000000000066666ddddddddddd
0777770000000000011110001000000100000000000000000e000222000000000000000004440440000000000000000000000000000000000000000000000000
777776600000000011111100110000110000000000000000020022220111111000000000044444400000000000000000080008000c000c000b000b0009000900
5775566000777770101111101110001100bbbb00064446d000e222220111111000000000022222200000000000000000088088000cc0cc000bb0bb0009909900
656677600757775700111111011001100b3773b06444644d000222220100000000000000444444440000000000000000088888000ccccc000bbbbb0009999900
000556000777577701122221011001000b3733b066a66ddd02e022220111011000000000000a0a000000000000000000088828000ccc1c000bbb3b0009994900
075756000077777022200002001e1e00b333333b6494644de00e0eee0011010000000000000000000000000000000000082828000c1c1c000b3b3b0009494900
0066600000707070000a0a0000011000b333333b6444644d002002220002200000000000002222000000000000000000082828000c1c1c000b3b3b0009494900
000000000000000000000000000100000bbbbbb066666ddd00e00222dd11d11d00000000044424400000000000000000080008000c000c000b000b0009000900
00000000000000000000000000000000000000000000000000000077444000670000440000070000000000000000000000000000000500000000000500000000
00000000060650500a0a90900d0d10100e0e202000000000000007765004400600044000070dd760000000000000000007775770000500000000000500000000
06666660066655500aa799900ddd11102eee222e0000000000007760050004000044000007dddd00000000000000000050557505000500000000000500000000
07767670066655500a7a99900dd1d1100eee2220000000000407760000504040044400000dd76d16000000000077777070777707000500000000000500000000
087667800066550000aa990000dd110020ee220e000000000477600000050040444000007d166110000000000757775700557500000500000000000500000000
0770077000565500009a9900001d1100002e22000000000000460000074050044447000000111160000000000777577700700700444444444444444444400000
07800870000550000009900000011000020220e00000000049044000647005040464474007611060000000000077777000700700444ff4f4f44ff4fff4400000
0000000000000000000000000000000000000000000000004400000006000054000464440000600000000000007070700077077044f444f4f4f4f4f4f4400000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000044fff4fff4f4f4fff4400000
000000000000000000000000000000000000000000000000111011101110111010100000111001100110111000000000000000004444f4f4f4f4f4f444400000
0000000001111110011111100111111001111110000000001010101010001010101000001010101010101110000000000000000044ff44f4f4ff44f444400000
00000000111111111111111111111111111111110000000011001100110011101100000011001010101010100000000000000000444444444444444444400000
00000000111111111111111111111111111111110000000010101010100010101010000010101010101010100000000000000000000000000000000000000000
0000000002ff2f1102ff2f1102ff2f1102ff2f110000000011101010111010101010000010101100110010100000000000000000000000000000000000000000
000000000efffef10efffef10ffffff10ffffff10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000001ffff1101ffff1101ffff1101ffff110000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000011ee111011ee111011ee111011ee1110000000000888800008888008800000000000000000000000000000000000000000000000000000000000000
000000001cccc1c11cccc1c11cccc1c11cccc1c10000000008800880088008808000000000000000000000000000000000000000000000000000000000000000
000000000ecccce10ecccce10ecccce10ecccce10000000008800880088008800000000000000000000000000000000000000000000000000000000000000000
000000000e111e000e111e000eefeee00e1110e00000000008800880088008800000000000000000000000000000000000000000000000000000000000000000
0000000000efe00000efe00000ccc0000eccc0e00000000008800880088008800000000000000000000000000000000000000000000000000000000000000000
0000000000cccc0000cccc0000cccc000fccccf00000000008800000088008800000000000000000000000000000000000000000000000000000000000000000
000000000ccccc0000ccccc000ccccc000ccccc00000000008800000088008800000000000000000000000000000000000000000000000000000000000000000
000000000cccccc00ccccccc0ccccccc0ccccccc0000000008808880088008800000000000000000000000044404400000000000000000777777777777777770
00000000000000000000000000000000000000000000000008800880088008800000000000000000000000044444400000000000000007747474447474777777
00000000000000000000000000000000000000000000000008800880088008800000000000000000000000022222200000000000000007747474777474777777
00000000000000000000000000000000000000000000000008800880088008800000000000000000000044444444444400000000000007744474447444777777
00000000000000000000000000000000000000000000000000888800008888000000000000000000000000000a0a000000000000000007747474777774777777
00000000000000000000000000000000000000000000000000000000000000008000000000000000000000000000000000000000000007747474447444774777
00000000000000000000000000000000000000000000000000000000000000008800000000000000000000002222000000000000000000777777777777777770
00000000000000000000000000000000000000000000008088880888880880888880000000000000000000044424400000000000000000000000077700000000
00000000000000000000000000000000000000000000800888088888808888888880000000000000000000444244440000000000000000000000007000000000
__label__
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
01003010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00030300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
13b01010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0b3b30b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000003000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000b300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000003010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000b00001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000003011000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000b100b3b000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000003b330b0300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000030100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000b1001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000030100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000000000b000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000030100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000b3000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000030010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000030000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000b3000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000030100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000000000b000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000030110000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
000000000000000b100b3b0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
0000000000000003b330b03000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000010000000000000000000000000000000000000000000000220220222202202222022022220220222202202200000000000000000
00000000000000000000000300000000001000000010000000100000001000000010000110110111101101111011011110110111101101100010000000000000
00000000000000000000000b30000000000010000000100000001000000010000000100000000000000000000000000000000000000000000000100000000000
00000000000000000000000301000000100000001000000010000000100000001000000202222022022220220222202202222022022220201000000000000000
000000000000000000000000b0000100010030100100301001003010010030100100301101111011011110110111101101111011011110100100301000000000
00000000000000000000000301100003000303030003030300030303000303030003030000000000000000000000000000000000000000030003030000000000
00000000000000000000000b100b3b0013b0101013b0101013b0101013b0101013b01012202202222022022220220222202202222022022013b0101000000000
000000000000000000000003b330b03b0b3b30bb0b3b30bb0b3b30bb0b3b30bb0b3b30b1101101111011011110110111101101111011011b0b3b30b000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000300000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b30000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000301000000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000b0000100
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000301100000
00000000000000000000000000000000000000000000000000077777777777777777777777777777777700000000000000000000000000000000000b100b3b00
000000000000000000000000000000000000000000000000007711771717117771171117711711777177700000000000000000000000000000000003b330b030
00000000000000000000000000000000000000000000000000771717171717171777177717171717717770000000000022222100000000000000000000000003
0000000000000000000000000000000000000000000000000077171717171717177711771717171771777000000000022222111000000000000000000000000b
00000000000000000000000000000000000000000000000000771717171717171717177717171717777770000000010200210020100000000000000000000003
00000000000000000000000000000000000000000000000000771117711717171117111711771717717770000000110122211210110000000000000000000000
00000000000000000000000000000000000000000000000000077777777777777777777777777777777700000001111011211101111000000000000000000003
0000000000000000000000000000000000000000000000000000000000000001117771100000000000000000000110001011010001100000000000000000000b
00000000000000000000000000000000000000000000000000000000000000000007100000000000000000000011100000000000011100000000000000000003
00000000000000000000000000000000000000000000000000000000000000000111110000000000000000000011000000000000001100000000000000000000
00000000000000000000000000000000000000000000000001000100000000000888000000000000000800000000000000000000000000000008000000000003
0000000000000000000000000000000000000000000000011111111000000000888800000000000080880000001100000000000000110000808800000000000b
00000000000000000000000000000000000000000000000000100000000000080080000000000000088900000021000000000000002100000889000000000003
00000000000000000000000000000000000000000000000011111100000000000666600000000000009a0000002100000000000000210000009a000000000000
00000000000000000000000000000000000000000000000010000000000000006655550000000000004500000021000000000000002100000045000000000003
0000000000000000000000000000000000000000000000011111111000000000656565600000000000450000002100000000000000210000004500000000000b
00000000000000000000000000000000000000000000000000001000000000006655550000000000004500000021000000000000002100000045000000000003
00000000000000000000000000000000000000000000000001111100000000000666600000000000000500000000000000000000000000000005000000000000
000000000000000000000000000000000000000000000000000000000000000000880000000000000000000000110000000000000011000000000003b330b030
00000000000000000000000000000000000000000000000000000000000000055665665000000000000000000021000000000000002100000000000b100b3b00
00000000000000000000000000000000000000000000000000000000000000506665660500000000000000000021000000000000002100000000000301100000
000000000000000000000000000000000000000000000000000000000000006085555006000000000000000000210000000000000021000000000000b0000100
00000000000000000000000000000000000000000000000000000000000000008666600000000000000000000021000000000000002100000000000301000000
00000000000000000000000000000000000000000000000000000000000000008588500000000000000000000000000000000000000000000000000b30000000
00000000000000000000000000000000000000000000000000000000000000008688600000000000000000000222200000000000022220000000000300000000
00000000000000000000000000000000000000000000000000000000000000000660660000000000000000001111110000000000111111000000000010000000
0b3b30bb0b3b30bb0b3b30bb0b3b30bb0b3b30bb0b3b30bb0b3b30b220220222202202222022022220220226dddddd26dddddd26dddddd222022022000000000
13b0101013b0101013b0101013b0101013b0101013b0101013b010111011011110110111101101111011011d22d2d21d22d2d21d22d2d2111011011000000000
000303030003030300030303000303030003030300030303000303000000000000000000000000000000000dd222221dd222221dd22222100000000000000000
010030100100301001003010010030100100301001003010010030120222202202222022022220220222202d2d22211d2d22211d2d2221120222202000000000
100000001000000010000000100000001000000010000000100000010111101101111011011110110111101d2221221d2221221d222122110111101000000000
000010000000100000001000000010000000100000001000000010000000000000000000000000000000000d2121211d2121211d212121100000000000000000
001000000010000000100000001000000010000000100000001000022022022220220222202202222022022d2212121d2212121d221212122022022000000000
00000000000000000000000000000000000000000000000000000001101101111011011110110111101101121111111211111112111111111011011000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000

__gff__
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000030103030500030303030303030000011003030300000303030303030300001003030303001003030303030303000010030303080909081010000003030003
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000080000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
__map__
4400000000000000000000000000000000000000000000000000000000000000000000000000000000720000000000000000000000000000616161616161000000000000000000000000000000000000000000000000000000000000000000000000000000004c00000000007261000000000000000000000000000000000000
4400000000000000000000000000000000000000000000000000000000000000000000000000000000007261000000000000000000000073000000000000726100000000000000000000000000000000000000000000000000000000000061617300000000004c00000000000000726100000000000000000000000000000000
4400000000000000000000000000000000000000000000000000000000000000000000000000000000000000720000000000000000007300000000000000000072000000000000000000000061616161610000000000000000000000007300000000000000004c00000000000000000072000000000000000000000000000000
4400000000000000000000000000000000000000000000000000000000000000000000000000000000000000007200000000000000630000000000000000000000726161000000000061617300000000007261616100000000006161730000000000000000004c00000000000000000000620000000000000000000000000000
4400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000072610000000000630000000000000000000000000000726161617300000000000000000000000072000000730000000000000000000000004c00000000000000000000720000000000000000000000000000
4400000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007200000000630000000000000000000000000000000000000000000000000000000000000000720063000000000000000000000000004c00000000000000000000007261616161614848484848610000
440000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007261000063424242525a5a5a5a5a5300000000000000000000000000000000000000000000006263000000000000000000000000004c00000000000000000000000000000000000000000000007200
440000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000620063000000620000000000005a5a5300000000000000000000000000000000000000006263000000000000000000525300004c0000525a5a53000000000000000000004f00007b7c7d000062
440000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000620063000000620000000000000000005300000000000000000000000000000000000000626300000000000000525a006300004d00006200006300000000000000004f00000078460046780062
44000000000000000000000000000000000000000000000000000000000000000000000000000000004b00000000000000000062006300000062000000000000000000005a530000000000000000000000000000000000727300000075525a5a000000630000000000620000005a530000000000000000000000560056005200
44000000000000000000000000000000000000000000000000000000000000000000000000000000004c000000000000000000620063000000620000000000000000000000005a5a5300000000000000000000000000000000000000756200000000006300000000006200000000005a5a5a5a5a5a5a48484848494949480000
440000000000000000000000000000000000000000525300000000000000000000000000004b0000004c0000000000000000007261730000006200000000000000000000000000006300000000004b00000000000000000000000000756200000000006300000000006200000000000000000000000000000044000000440000
440000000000000000000000000000000000525a5a0000530000000000000000004b0000005a0000004c5300000000000000000000000000006200000000000000000000000000006300000000004c00000000000000000000000000756200000000006300000000006200000000000000000000000000000044000000440000
4400000000000000000000000000000000520000000000630000000000000000004c000000000000004d0053000000000000000000000000006200000000000000000000000000006300000000004c000000000000525a5a5a5a5a5a5a0000000000006300004b00006200000000000000000000000000000044000000440000
440000000000000000525a5a5a53000000620000000000630000000000000000004d000000000000005200005a5300000000000000000000520000000000000000000000000000006374747474744d74747474747462000000000000000000000000006374744c74746200000000000000000000000000000044000000440000
5a5a5a5a5a5a5a5a5a00000000005a5a5a000000000000005a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a5a00000000005a5a5a5a5a5a5a5a5a5a00000000000000000000000000000000005a5a5a5a5a5a5a5a5a5a5a5a0000000000000000000000000000005a5a5a5a5a0000000000000000000000000000000044000000440000
4040404040404040404040404040404000000000000000000000000000000000000000000000000000000000000000000069696969696969696969696969696969000000000000000000000000000000000000000000000000000000000000000000004949494949494949494949494949494900000000000044000000440000
000000000000000000710000000000000000000000000000000000000000000000000000000000000000000000000000000000006c00000000000000006c000069000000000000000000000000000000000000000000000000000000000000000000004900000000000000000000000000004900000000000044000000440000
000000000000000000710000000000000000000000000000000000000000000000000000000000000000000000000000000000006c00000000000000006c000069000000000000000000000000000000000000000000000000000000000000000000004c00000000000000000000000000004c00000000000044000000440000
000000000000000000710000000000000000000000000000000000000000000000000000000000000000000000000000000000006d00000000000000006d000069000000000000000000000000000000000000000000000000000000000000000000004c00000000000000000000000000004c00000000000044000000440000
0000000000000000007100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000069000000000000000000000000000000000000000000000000000000000000000000004c00000000000000000000000000004c00000000000044000000440000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000079000000000000000079000069000000000000000000000000000000000000000000000000000000000000000000004c00000000000000000000000000004c00000000000044000000440000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006b00000000000000006b000069000000000000000000000000000000000000000000000000000000000000000000004c00000000000000000000000000004c00000000000044000000440000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006c00000000000000006c000069000000000000000000000000000000000000000000000000000000000000000000004800000000000000000000000000004800000000000044000000440000
000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006c00000000000000006c000069000000000000000000000000000000000000000000000000000000000000000000004800000000000000000000000000004800000000000000000000000000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000068000000000000000068000069000000000000000000000000000000000000000000000000000000000000000000004800000000000000000000000000004800000000000000000000000000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000069000000000000000000000000000000000000000000000000000000000000000000004800000000000000000000000000004800000000000000000000000000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004800000000000000000000000000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000007800000000780000000000000078000000000078000000000078000000000078000078000000000000000000004800000000000000000000000000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004800000000000000000000000000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000071545464004800000000000000000000000000
0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000006a6a6a6a6a6a6a6a6a6a6a6a6a6a6a6a6a6a6a446a6a446a44446a444444444a444a4a4a44444a44444a444a444a4a4a444a4a7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f7f00000000000000000000000000
__sfx__
090700000f6510b2000964112200006611500000000000000b4300b43008430084300543005430000000000024640306403c64018640006350000000000000000000000000000000000000000000000000000000
001400000c1750c1150c1150c1151217512115161751717517115161751217512115181751811512175121150c1750c1150c1150c1151217512115161751717517115121750c1750c1150a1750a1150917508175
00140000141751411514115141150f1750f1150f1150d1750d1150d1150d115091750817508115071750711506175061150611506115121751117511115101751011510115101150e1750d1750d1150c1750c115
00140000091750911509175091151417514175141150f1750f1150f1750a1750a11508175081150b1750c1750d1750d1150d17519175191150d1750d1150d1750d1150d1750d1150d1750f1750d1750a17509175
00300000100401c00010040220000e040270000e0402f0000c040000000c040000000804000000080400000007040070400704007040070400704507040070400704507000070250000007000070000000000000
001a0000094320943215432154322143221432244322143209432094321543209432214321843224432214320b4320b4321743217432234322343226432234320843208432144320843220432174322343220432
000d00200c640000003c6000000034630000000c6400c6000c65000000000000000034630000000c6400000000000000000c6400000034630000000c640000000c650000000c6000000034640000000000000000
001a00000930000000000000000021340000001c34021340213001c3401800021340213001c3401c300183401b3401800018000180001b340180001e3401c3001c3401800018000183401a340180001834018000
001a0020094201542009420184200942015420094200c420094201542009420184200942015420094200c4200b420174200b4201a4200b420174200b4200e4200842014420084201442004420104200442010420
000d00001535515355000050935509355000001535515355000000000000000000001535515355183550000015355153550000009355093550000015355153550000000000000001530018355183551735515300
001a00000930000000000000000021340000001c34021340093001c340000002134021300233401c30007300243400000023300233401b30000000243401c3001c3002334000000233402434023340203401c340
000d00001735517355000050b3550b3550000017355173550000000000000000000017355173551a3550000014355143550000008355083550000014355143550000000000000001530014355143551735515300
00300000130401c00013040220001204027000120402f000110400000011040000001104000000110400000018040180401804018040180401804018000180000000000000000000000000000000000000000000
00300000180401c00018040220001504027000150402f000150401600015040000001404000000140400000010040100401004010040100401004510045100301004500000000000000000000000000000000000
001a0020000001c3002843017300244301730023430000002143000000234300000024430000002843021400274301c3002343017300244001740023430244302943000000284300000029430000002843021430
001a00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000346350000034635
01240000000012d631216311563109631306710060100000000003460000000000000000000000000000000000000000000000000000000000000000000000000000000000000003460000000346000000000000
003400000030010300043001c30004300283001c30021300093001c3000030021300213001c3001c300183001b3000030000300003001b300003001e3001c3001c3000030000300183001a300003001830000300
001a0000093000000000000000001c3402134024340283400930024340000002130023340243402334018300263400000023340263401b3002334026340233402834023300263401830024340233401830021340
001a0000093000000000000000001c3402134024340283400930029340000002934028340263402434018300263400000023340263401b3002334026340233402834000000293401830028340000002634000000
000d00080c60000000000000000034625000000c6000c600346240000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
031a000000000396712d6712167115631096210960100000246001e00020000220002500027000000000000000000000000000000000000000000000000000000000000000000003460000000346000000000000
05100000385603840038560385001550509505155050b5052b33328333243330b505175050b505175050c50529675185050c505185050c5050e50508505145050850514505045051050504505105050050000000
00140000081750811508115081150f1750f1150f1150d1750d1150d1150d1150d17514175141151317513115121751211512115121150d1750a1750a1150f1750f1150f1150f115091750a1750a1150917509115
011400200a0000f000000000a0000f000000000a0000f0000a0000a0000f000000000e000000000e000000000a0000f000000000a0000f000000000a0000f0000a0000a0000f0000000010000000001000000000
0104000025d712bd7131d7137d713fd711720017200182001d6612066122661316613f6611420014200000001245012450124500a4500a4500a45000000000000000000000000000000000000000000000000000
490700000030010361043002836104300343610000000000336550000027655000001b655000000f6550000000000000000000000000000000000000000000000000000000000000000000000000000000000000
590900003a5453f545375453f54500500000003b6611562139631126113b6110d65138655006000050005621096300b6400c6500a670076500463000620005003f67533655266551b655000002a6640000000000
011900000945509455154551545505455054551145511455094550945515455154550c4550c455184551845509455094551545515455054550545511455114550445504455104551045508455084551445514455
0019000024352003002330021350233520c30021350000002135200300203001d350203520c300213500000024352003002335221350233520c300213500000021352003002030023350203520c3001d35000000
001900000932209322153221532205322053221132211322093220932215322153220c3220c322183221832209322093221532215322053220532211322113220432204322103221032208322083221432214322
c01000200061600616006160061600616006160061600616006160061600616006160061600616006160061600616006160061600616006160061600616006160061600616006160061600616006160061600616
d61000000002000020000200002000020000200002000020000200002000020000200002000020000200002000020000200002000020000200002000020000200002000020000200002000020000200002000020
1c1600001e3501e350213502134225350233501c3501e3501e3421e332253502534223350213501e350213502134221332213221c3501e35021350233502135023350233421e3501e342213501c3501c34219350
481000000202002020020200202002020020200202002020020200202002020020200202002020020200202002020020200202002020020200202002020020200202002020020200202002020020200202002020
c00a000037616306162c61624616216161e6161961616616146161761612616106160f6160e6160d6160c616096160b6160a6160a616096160861608616076160661605616056160461603616036160061600616
001600000644506445064451244512445124450644512445064450644506445124451244512445064451244504445044450444510445104451044504445104450444504445044451044510445104450444510445
001600000244502445024450e4450e4450e445024450e4450244502445024450e4450e4450e445024450e4450144501445014450d4450d4450d445014450d4450444504445044451044510445104450444510445
1c2c0000193421933219322213502535025342253322532223350233422333223322213502134220350213501e3501e3421e3321e3221e3121e3121e312213501d3501d3321d3221d3121e3501e3422135021342
1c2c0000253502533228350283322535025332213502335021350213321e3501e33219350193321e3502135023350233322332223312283502535023350213502535023350233322332225350253322835028352
201600002a4602a4522a4522a4422a4422a4322a4322a4222a4222a4122a4122a4122a412284602a4602a4522d4602d4522c4602c4522c4522c4422c4422c4222a4602a4522a4422846028462284622c4602c462
895800000606006050060400607105070050500504005071060600605006040060710807008050080400807106060060500604006071050600505005040050710606006050060400607109070090500904009071
4a1600201c6753f6753c6753f6751c6753f6753c6753f6751c6753f6753c6753f6751c6753f6753c6753f6751c6753f6753c6753f6751c6753f6753c6753f6751c6753f6753c6753f6753c6753c6753c6753f675
000b00002e0112d0212c0212b0312a03129041280412705126051250512404123041220411c6551c6001c600260001c6001c600230002200121001200011f0011e0011d0011c0011b0011a001190011800117001
483200002867534635346253461534600006050060500605006050060500605006050060500605006050060500605006050060500605006050060500605006050060500605006053460000605006050000000000
885800000606006050060400607105070050500504005071010600105001040010710407004050040400407106060060500604006071050600505005040050710106001050010400107104070040500404004071
01140000081750811508175081150f1750f1150f1150d1750d1150d1150d11509175081750d175081750811506175061150a1750a1150a1150b1750b1150c1750d1750d115111751117512175121151317513115
012c002012565155651956515565125651556519565155651156515565195651556511565155651956515565125651556519565155651256515565195651556514565175651b5651756515565195651c56519565
010f00003a061390613805137051360513505134051330513204131041300412f0412e0412d0312c0312b0312a031290312803127021260212502124021230212202121011200111f0111e0111d0111c0111b011
000900003a1453f1453f1240000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
d50b00201e47021450254502a450231000010021100211001e43021420254202a42023100281002a100001002a1002d1003110036100231000010021100001002a10025100231002810028100281002c10000100
211600002a4602a4522a4522a4422a4422846025460254522d4602d4522d4522d4422d4422d43231460314522f4602f4522f4422d4602c4602c4522c4422c4322a4602a4522a4422846028452284422843228422
01160000094000940009400154001540015400094001540009400094000940015400154001540009400154000140001400014000d4000d4000d400014000d4000440004400044001040010400104000440010400
01160000094450944509445154451544515445094451544509445094450944515445154451544509445154450144501445014450d4450d4450d445014450d4450444504445044451044510445104450444510445
012c002006435124351e4351243506435124351e4351243504435104351c4351043504435104351c43510435024450e4451a4450e445024450e4451a4450e445014450d445194450d44504445104451c44510445
042e00000c0501005018050100500b0501005017050100500905010050150501005007050100501305010050050500c050110500c050040500c050100500c050070500e050130500e05009050100501505010050
012e0000050000c000110000c000040000c000100000c000070000e000130000e00009000100001500010000050000c000110000c000040000c000100000c000070000e000130000e00009000100001500010000
0307000024670306503c6601867000645006450030015300133000030000300003001d30011300003000030017300183000030000300003000030000300003001c30018300183001c3001f300003000030000000
492e00001f3301f3201f310213301f3301f3201f3101d3301c3301c3201a3301a3201d3301d3201c3301c3201f3301f3201f310213301f3301f3201f3101d3301c3301c3201a3301a320183301a3301c3301a330
492e00001f3301f3201f310213301f3301f3201f3101d3302433024320233302332021330213201f3301d3301f3301f3201f310213301f3301f3201f310243302b3302b3202b3102b3102b310293302833026330
492e00002433024320243102431024310243102633026320243302432024310263302833028320263302632024330243202431024310243102431026330263202433024320213301f33028330243302633024330
492e00002432024310243102431024310243102633026320243302432024310263302833028320263302632024330243202431026330243302432024310263302433024320213302333024330233301f3301d330
011700002430024300243002430024300243002630026300243002430024300263002830028300263002630000000000000000000000000000000000000000000000000000000000000000000000000000000000
490b002000670246002460024600046002460004675266002167024600046751f6002860004600046752460000600006000467500600216700060004675006002167000600046750060021670006000060000600
__music__
01 05065644
00 05060f44
00 07080644
00 0a08060f
00 0e081451
00 0e081451
00 12080644
00 1308064f
00 09144344
00 0b144344
00 09140e51
00 0b140e44
02 0e140551
01 171f6d44
00 2e1f6d44
00 021f4344
00 011f4344
02 031f4344
01 1c5d1e44
02 1c1d1e44
04 040c0d44
01 24212a76
00 25212a44
00 24212a76
00 25212a44
00 36262a76
00 36272a76
00 24282a76
00 35332a76
00 24282a76
02 35332a76
01 291f5e44
00 291f2f7f
00 293f2f44
00 323f2f44
02 723f2f44
00 727f6f44
00 723f2f44
04 6b42432b
00 23204344
01 1f204344
02 1f224344
01 64616a76
02 65626a44
00 305e4344
00 65696a44
00 64726a44
00 74736a44
03 77374344
01 77374344
00 3a377644
00 3b374344
01 3c377644
04 3d374344
01 7e774344
01 7e784344
01 7e784344
04 41784344
01 040c0d44

