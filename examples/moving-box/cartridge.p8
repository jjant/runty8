pico-8 cartridge // http://www.pico-8.com
version 41
__lua__
local x=0
local y0=16

function _init()
	cls()
	poke(0x5f2d, 0x01)
end

function _update()
 if x==0 then
   y0+=20
 end

 local y = btnp(❎) and y0 - 16 or y0

 rect(x, y0, x, y, 8)
  
 x=(x+1)%128
end
__gfx__
00000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000171000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700700177100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000177710000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00077000177770000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00700700177110000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
00000000011710000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
