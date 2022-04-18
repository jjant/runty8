-- ~celeste~
-- matt thorson + noel berry

-- globals --
-------------

room = { x=0, y=0 }
objects = {}
types = {}
freeze=0
shake=0
will_restart=false
delay_restart=0
got_fruit={}
has_dashed=false
sfx_timer=0
has_key=false
pause_player=false
flash_bg=false
music_timer=0

k_left=0
k_right=1
k_up=2
k_down=3
k_jump=4
k_dash=5

-- entry point --
-----------------

function _init()
    title_screen()
end

function title_screen()
    got_fruit = {}
    for i=0,29 do
        add(got_fruit,false) end
    frames=0
    deaths=0
    max_djump=1
    start_game=false
    start_game_flash=0
    music(40,0,7)
   
    load_room(7,3)
end

function begin_game()
    frames=0
    seconds=0
    minutes=0
    music_timer=0
    start_game=false
    music(0,0,7)
    load_room(0,0)
end

function level_index()
    return room.x%8+room.y*8
end

function is_title()
    return level_index()==31
end

-- effects --
-------------

clouds = {}
for i=0,16 do
    add(clouds,{
        x=rnd(128),
        y=rnd(128),
        spd=1+rnd(4),
        w=32+rnd(32)
    })
end

particles = {}
for i=0,24 do
    add(particles,{
        x=rnd(128),
        y=rnd(128),
        s=0+flr(rnd(5)/4),
        spd=0.25+rnd(5),
        off=rnd(1),
        c=6+flr(0.5+rnd(1))
    })
end

dead_particles = {}

-- player entity --
-------------------

player = 
{
    init=function(this) 
        this.p_jump=false
        this.p_dash=false
        this.grace=0
        this.jbuffer=0
        this.djump=max_djump
        this.dash_time=0
        this.dash_effect_time=0
        this.dash_target={x=0,y=0}
        this.dash_accel={x=0,y=0}
        this.hitbox = {x=1,y=3,w=6,h=5}
        this.spr_off=0
        this.was_on_ground=false
        create_hair(this)
    end,
    update=function(this)
        if (pause_player) return
       
        local input = btn(k_right) and 1 or (btn(k_left) and -1 or 0)
       
        -- spikes collide
        if spikes_at(this.x+this.hitbox.x,this.y+this.hitbox.y,this.hitbox.w,this.hitbox.h,this.spd.x,this.spd.y) then
         kill_player(this) end
         
        -- bottom death
        if this.y>128 then
            kill_player(this) end

        local on_ground=this.is_solid(0,1)
        local on_ice=this.is_ice(0,1)
       
        -- smoke particles
        if on_ground and not this.was_on_ground then
         init_object(smoke,this.x,this.y+4)
        end

        local jump = btn(k_jump) and not this.p_jump
        this.p_jump = btn(k_jump)
        if (jump) then
            this.jbuffer=4
        elseif this.jbuffer>0 then
         this.jbuffer-=1
        end
       
        local dash = btn(k_dash) and not this.p_dash
        this.p_dash = btn(k_dash)
       
        if on_ground then
            this.grace=6
            if this.djump<max_djump then
             psfx(54)
             this.djump=max_djump
            end
        elseif this.grace > 0 then
         this.grace-=1
        end

        this.dash_effect_time -=1
  if this.dash_time > 0 then
   init_object(smoke,this.x,this.y)
      this.dash_time-=1
      this.spd.x=appr(this.spd.x,this.dash_target.x,this.dash_accel.x)
      this.spd.y=appr(this.spd.y,this.dash_target.y,this.dash_accel.y)  
  else

            -- move
            local maxrun=1
            local accel=0.6
            local deccel=0.15
           
            if not on_ground then
                accel=0.4
            elseif on_ice then
                accel=0.05
                if input==(this.flip.x and -1 or 1) then
                    accel=0.05
                end
            end
       
            if abs(this.spd.x) > maxrun then
             this.spd.x=appr(this.spd.x,sign(this.spd.x)*maxrun,deccel)
            else
                this.spd.x=appr(this.spd.x,input*maxrun,accel)
            end
           
            --facing
            if this.spd.x!=0 then
                this.flip.x=(this.spd.x<0)
            end

            -- gravity
            local maxfall=2
            local gravity=0.21

      if abs(this.spd.y) <= 0.15 then
       gravity*=0.5
            end
       
            -- wall slide
            if input!=0 and this.is_solid(input,0) and not this.is_ice(input,0) then
             maxfall=0.4
             if rnd(10)<2 then
                 init_object(smoke,this.x+input*6,this.y)
                end
            end

            if not on_ground then
                this.spd.y=appr(this.spd.y,maxfall,gravity)
            end

            -- jump
            if this.jbuffer>0 then
             if this.grace>0 then
              -- normal jump
              psfx(1)
              this.jbuffer=0
              this.grace=0
                    this.spd.y=-2
                    init_object(smoke,this.x,this.y+4)
                else
                    -- wall jump
                    local wall_dir=(this.is_solid(-3,0) and -1 or this.is_solid(3,0) and 1 or 0)
                    if wall_dir!=0 then
                     psfx(2)
                     this.jbuffer=0
                     this.spd.y=-2
                     this.spd.x=-wall_dir*(maxrun+1)
                     if not this.is_ice(wall_dir*3,0) then
                         init_object(smoke,this.x+wall_dir*6,this.y)
                        end
                    end
                end
            end
       
            -- dash
            local d_full=5
            local d_half=d_full*0.70710678118
       
            if this.djump>0 and dash then
             init_object(smoke,this.x,this.y)
             this.djump-=1       
             this.dash_time=4
             has_dashed=true
             this.dash_effect_time=10
             local v_input=(btn(k_up) and -1 or (btn(k_down) and 1 or 0))
             if input!=0 then
              if v_input!=0 then
               this.spd.x=input*d_half
               this.spd.y=v_input*d_half
              else
               this.spd.x=input*d_full
               this.spd.y=0
              end
             elseif v_input!=0 then
                 this.spd.x=0
                 this.spd.y=v_input*d_full
             else
                 this.spd.x=(this.flip.x and -1 or 1)
              this.spd.y=0
             end
            
             psfx(3)
             freeze=2
             shake=6
             this.dash_target.x=2*sign(this.spd.x)
             this.dash_target.y=2*sign(this.spd.y)
             this.dash_accel.x=1.5
             this.dash_accel.y=1.5
            
             if this.spd.y<0 then
              this.dash_target.y*=.75
             end
            
             if this.spd.y!=0 then
              this.dash_accel.x*=0.70710678118
             end
             if this.spd.x!=0 then
              this.dash_accel.y*=0.70710678118
             end          
            elseif dash and this.djump<=0 then
             psfx(9)
             init_object(smoke,this.x,this.y)
            end
       
        end
       
        -- animation
        this.spr_off+=0.25
        if not on_ground then
            if this.is_solid(input,0) then
                this.spr=5
            else
                this.spr=3
            end
        elseif btn(k_down) then
            this.spr=6
        elseif btn(k_up) then
            this.spr=7
        elseif (this.spd.x==0) or (not btn(k_left) and not btn(k_right)) then
            this.spr=1
        else
            this.spr=1+this.spr_off%4
        end
       
        -- next level
        if this.y<-4 and level_index()<30 then next_room() end
       
        -- was on the ground
        this.was_on_ground=on_ground
       
    end, --<end update loop
   
    draw=function(this)
   
        -- clamp in screen
        if this.x<-1 or this.x>121 then 
            this.x=clamp(this.x,-1,121)
            this.spd.x=0
        end
       
        set_hair_color(this.djump)
        draw_hair(this,this.flip.x and -1 or 1)
        spr(this.spr,this.x,this.y,1,1,this.flip.x,this.flip.y)       
        unset_hair_color()
    end
}

psfx=function(num)
 if sfx_timer<=0 then
  sfx(num)
 end
end

create_hair=function(obj)
    obj.hair={}
    for i=0,4 do
        add(obj.hair,{x=obj.x,y=obj.y,size=max(1,min(2,3-i))})
    end
end

set_hair_color=function(djump)
    pal(8,(djump==1 and 8 or djump==2 and (7+flr((frames/3)%2)*4) or 12))
end

draw_hair=function(obj,facing)
    local last={x=obj.x+4-facing*2,y=obj.y+(btn(k_down) and 4 or 3)}
    foreach(obj.hair,function(h)
        h.x+=(last.x-h.x)/1.5
        h.y+=(last.y+0.5-h.y)/1.5
        circfill(h.x,h.y,h.size,8)
        last=h
    end)
end

unset_hair_color=function()
    pal(8,8)
end

player_spawn = {
    tile=1,
    init=function(this)
     sfx(4)
        this.spr=3
        this.target= {x=this.x,y=this.y}
        this.y=128
        this.spd.y=-4
        this.state=0
        this.delay=0
        this.solids=false
        create_hair(this)
    end,
    update=function(this)
        -- jumping up
        if this.state==0 then
            if this.y < this.target.y+16 then
                this.state=1
                this.delay=3
            end
        -- falling
        elseif this.state==1 then
            this.spd.y+=0.5
            if this.spd.y>0 and this.delay>0 then
                this.spd.y=0
                this.delay-=1
            end
            if this.spd.y>0 and this.y > this.target.y then
                this.y=this.target.y
                this.spd = {x=0,y=0}
                this.state=2
                this.delay=5
                shake=5
                init_object(smoke,this.x,this.y+4)
                sfx(5)
            end
        -- landing
        elseif this.state==2 then
            this.delay-=1
            this.spr=6
            if this.delay<0 then
                destroy_object(this)
                init_object(player,this.x,this.y)
            end
        end
    end,
    draw=function(this)
        set_hair_color(max_djump)
        draw_hair(this,1)
        spr(this.spr,this.x,this.y,1,1,this.flip.x,this.flip.y)
        unset_hair_color()
    end
}
add(types,player_spawn)

spring = {
    tile=18,
    init=function(this)
        this.hide_in=0
        this.hide_for=0
    end,
    update=function(this)
        if this.hide_for>0 then
            this.hide_for-=1
            if this.hide_for<=0 then
                this.spr=18
                this.delay=0
            end
        elseif this.spr==18 then
            local hit = this.collide(player,0,0)
            if hit ~=nil and hit.spd.y>=0 then
                this.spr=19
                hit.y=this.y-4
                hit.spd.x*=0.2
                hit.spd.y=-3
                hit.djump=max_djump
                this.delay=10
                init_object(smoke,this.x,this.y)
               
                -- breakable below us
                local below=this.collide(fall_floor,0,1)
                if below~=nil then
                    break_fall_floor(below)
                end
               
                psfx(8)
            end
        elseif this.delay>0 then
            this.delay-=1
            if this.delay<=0 then 
                this.spr=18 
            end
        end
        -- begin hiding
        if this.hide_in>0 then
            this.hide_in-=1
            if this.hide_in<=0 then
                this.hide_for=60
                this.spr=0
            end
        end
    end
}
add(types,spring)

function break_spring(obj)
    obj.hide_in=15
end

balloon = {
    tile=22,
    init=function(this) 
        this.offset=rnd(1)
        this.start=this.y
        this.timer=0
        this.hitbox={x=-1,y=-1,w=10,h=10}
    end,
    update=function(this) 
        if this.spr==22 then
            this.offset+=0.01
            this.y=this.start+sin(this.offset)*2
            local hit = this.collide(player,0,0)
            if hit~=nil and hit.djump<max_djump then
                psfx(6)
                init_object(smoke,this.x,this.y)
                hit.djump=max_djump
                this.spr=0
                this.timer=60
            end
        elseif this.timer>0 then
            this.timer-=1
        else 
         psfx(7)
         init_object(smoke,this.x,this.y)
            this.spr=22 
        end
    end,
    draw=function(this)
        if this.spr==22 then
            spr(13+(this.offset*8)%3,this.x,this.y+6)
            spr(this.spr,this.x,this.y)
        end
    end
}
add(types,balloon)

fall_floor = {
    tile=23,
    init=function(this)
        this.state=0
        this.solid=true
    end,
    update=function(this)
        -- idling
        if this.state == 0 then
            if this.check(player,0,-1) or this.check(player,-1,0) or this.check(player,1,0) then
                break_fall_floor(this)
            end
        -- shaking
        elseif this.state==1 then
            this.delay-=1
            if this.delay<=0 then
                this.state=2
                this.delay=60--how long it hides for
                this.collideable=false
            end
        -- invisible, waiting to reset
        elseif this.state==2 then
            this.delay-=1
            if this.delay<=0 and not this.check(player,0,0) then
                psfx(7)
                this.state=0
                this.collideable=true
                init_object(smoke,this.x,this.y)
            end
        end
    end,
    draw=function(this)
        if this.state!=2 then
            if this.state!=1 then
                spr(23,this.x,this.y)
            else
                spr(23+(15-this.delay)/5,this.x,this.y)
            end
        end
    end
}
add(types,fall_floor)

function break_fall_floor(obj)
 if obj.state==0 then
     psfx(15)
        obj.state=1
        obj.delay=15--how long until it falls
        init_object(smoke,obj.x,obj.y)
        local hit=obj.collide(spring,0,-1)
        if hit~=nil then
            break_spring(hit)
        end
    end
end

smoke={
    init=function(this)
        this.spr=29
        this.spd.y=-0.1
        this.spd.x=0.3+rnd(0.2)
        this.x+=-1+rnd(2)
        this.y+=-1+rnd(2)
        this.flip.x=maybe()
        this.flip.y=maybe()
        this.solids=false
    end,
    update=function(this)
        this.spr+=0.2
        if this.spr>=32 then
            destroy_object(this)
        end
    end
}

fruit={
    tile=26,
    if_not_fruit=true,
    init=function(this) 
        this.start=this.y
        this.off=0
    end,
    update=function(this)
     local hit=this.collide(player,0,0)
        if hit~=nil then
         hit.djump=max_djump
            sfx_timer=20
            sfx(13)
            got_fruit[1+level_index()] = true
            init_object(lifeup,this.x,this.y)
            destroy_object(this)
        end
        this.off+=1
        this.y=this.start+sin(this.off/40)*2.5
    end
}
add(types,fruit)

fly_fruit={
    tile=28,
    if_not_fruit=true,
    init=function(this) 
        this.start=this.y
        this.fly=false
        this.step=0.5
        this.solids=false
        this.sfx_delay=8
    end,
    update=function(this)
        --fly away
        if this.fly then
         if this.sfx_delay>0 then
          this.sfx_delay-=1
          if this.sfx_delay<=0 then
           sfx_timer=20
           sfx(14)
          end
         end
            this.spd.y=appr(this.spd.y,-3.5,0.25)
            if this.y<-16 then
                destroy_object(this)
            end
        -- wait
        else
            if has_dashed then
                this.fly=true
            end
            this.step+=0.05
            this.spd.y=sin(this.step)*0.5
        end
        -- collect
        local hit=this.collide(player,0,0)
        if hit~=nil then
         hit.djump=max_djump
            sfx_timer=20
            sfx(13)
            got_fruit[1+level_index()] = true
            init_object(lifeup,this.x,this.y)
            destroy_object(this)
        end
    end,
    draw=function(this)
        local off=0
        if not this.fly then
            local dir=sin(this.step)
            if dir<0 then
                off=1+max(0,sign(this.y-this.start))
            end
        else
            off=(off+0.25)%3
        end
        spr(45+off,this.x-6,this.y-2,1,1,true,false)
        spr(this.spr,this.x,this.y)
        spr(45+off,this.x+6,this.y-2)
    end
}
add(types,fly_fruit)

lifeup = {
    init=function(this)
        this.spd.y=-0.25
        this.duration=30
        this.x-=2
        this.y-=4
        this.flash=0
        this.solids=false
    end,
    update=function(this)
        this.duration-=1
        if this.duration<= 0 then
            destroy_object(this)
        end
    end,
    draw=function(this)
        this.flash+=0.5

        print("1000",this.x-2,this.y,7+this.flash%2)
    end
}

fake_wall = {
    tile=64,
    if_not_fruit=true,
    update=function(this)
        this.hitbox={x=-1,y=-1,w=18,h=18}
        local hit = this.collide(player,0,0)
        if hit~=nil and hit.dash_effect_time>0 then
            hit.spd.x=-sign(hit.spd.x)*1.5
            hit.spd.y=-1.5
            hit.dash_time=-1
            sfx_timer=20
            sfx(16)
            destroy_object(this)
            init_object(smoke,this.x,this.y)
            init_object(smoke,this.x+8,this.y)
            init_object(smoke,this.x,this.y+8)
            init_object(smoke,this.x+8,this.y+8)
            init_object(fruit,this.x+4,this.y+4)
        end
        this.hitbox={x=0,y=0,w=16,h=16}
    end,
    draw=function(this)
        spr(64,this.x,this.y)
        spr(65,this.x+8,this.y)
        spr(80,this.x,this.y+8)
        spr(81,this.x+8,this.y+8)
    end
}
add(types,fake_wall)

key={
    tile=8,
    if_not_fruit=true,
    update=function(this)
        local was=flr(this.spr)
        this.spr=9+(sin(frames/30)+0.5)*1
        local is=flr(this.spr)
        if is==10 and is!=was then
            this.flip.x=not this.flip.x
        end
        if this.check(player,0,0) then
            sfx(23)
            sfx_timer=10
            destroy_object(this)
            has_key=true
        end
    end
}
add(types,key)

chest={
    tile=20,
    if_not_fruit=true,
    init=function(this)
        this.x-=4
        this.start=this.x
        this.timer=20
    end,
    update=function(this)
        if has_key then
            this.timer-=1
            this.x=this.start-1+rnd(3)
            if this.timer<=0 then
             sfx_timer=20
             sfx(16)
                init_object(fruit,this.x,this.y-4)
                destroy_object(this)
            end
        end
    end
}
add(types,chest)

platform={
    init=function(this)
        this.x-=4
        this.solids=false
        this.hitbox.w=16
        this.last=this.x
    end,
    update=function(this)
        this.spd.x=this.dir*0.65
        if this.x<-16 then this.x=128
        elseif this.x>128 then this.x=-16 end
        if not this.check(player,0,0) then
            local hit=this.collide(player,0,-1)
            if hit~=nil then
                hit.move_x(this.x-this.last,1)
            end
        end
        this.last=this.x
    end,
    draw=function(this)
        spr(11,this.x,this.y-1)
        spr(12,this.x+8,this.y-1)
    end
}

message={
    tile=86,
    last=0,
    draw=function(this)
        this.text="-- celeste mountain --#this memorial to those# perished on the climb"
        if this.check(player,4,0) then
            if this.index<#this.text then
             this.index+=0.5
                if this.index>=this.last+1 then
                 this.last+=1
                 sfx(35)
                end
            end
            this.off={x=8,y=96}
            for i=1,this.index do
                if sub(this.text,i,i)~="#" then
                    rectfill(this.off.x-2,this.off.y-2,this.off.x+7,this.off.y+6 ,7)
                    print(sub(this.text,i,i),this.off.x,this.off.y,0)
                    this.off.x+=5
                else
                    this.off.x=8
                    this.off.y+=7
                end
            end
        else
            this.index=0
            this.last=0
        end
    end
}
add(types,message)

big_chest={
    tile=96,
    init=function(this)
        this.state=0
        this.hitbox.w=16
    end,
    draw=function(this)
        if this.state==0 then
            local hit=this.collide(player,0,8)
            if hit~=nil and hit.is_solid(0,1) then
                music(-1,500,7)
                sfx(37)
                pause_player=true
                hit.spd.x=0
                hit.spd.y=0
                this.state=1
                init_object(smoke,this.x,this.y)
                init_object(smoke,this.x+8,this.y)
                this.timer=60
                this.particles={}
            end
            spr(96,this.x,this.y)
            spr(97,this.x+8,this.y)
        elseif this.state==1 then
            this.timer-=1
         shake=5
         flash_bg=true
            if this.timer<=45 and count(this.particles)<50 then
                add(this.particles,{
                    x=1+rnd(14),
                    y=0,
                    h=32+rnd(32),
                    spd=8+rnd(8)
                })
            end
            if this.timer<0 then
                this.state=2
                this.particles={}
                flash_bg=false
                new_bg=true
                init_object(orb,this.x+4,this.y+4)
                pause_player=false
            end
            foreach(this.particles,function(p)
                p.y+=p.spd
                line(this.x+p.x,this.y+8-p.y,this.x+p.x,min(this.y+8-p.y+p.h,this.y+8),7)
            end)
        end
        spr(112,this.x,this.y+8)
        spr(113,this.x+8,this.y+8)
    end
}
add(types,big_chest)

orb={
    init=function(this)
        this.spd.y=-4
        this.solids=false
        this.particles={}
    end,
    draw=function(this)
        this.spd.y=appr(this.spd.y,0,0.5)
        local hit=this.collide(player,0,0)
        if this.spd.y==0 and hit~=nil then
         music_timer=45
            sfx(51)
            freeze=10
            shake=10
            destroy_object(this)
            max_djump=2
            hit.djump=2
        end
       
        spr(102,this.x,this.y)
        local off=frames/30
        for i=0,7 do
            circfill(this.x+4+cos(off+i/8)*8,this.y+4+sin(off+i/8)*8,1,7)
        end
    end
}

flag = {
    tile=118,
    init=function(this)
        this.x+=5
        this.score=0
        this.show=false
        for i=1,count(got_fruit) do
            if got_fruit[i] then
                this.score+=1
            end
        end
    end,
    draw=function(this)
        this.spr=118+(frames/5)%3
        spr(this.spr,this.x,this.y)
        if this.show then
            rectfill(32,2,96,31,0)
            spr(26,55,6)
            print("x"..this.score,64,9,7)
            draw_time(49,16)
            print("deaths:"..deaths,48,24,7)
        elseif this.check(player,0,0) then
            sfx(55)
      sfx_timer=30
            this.show=true
        end
    end
}
add(types,flag)

room_title = {
    init=function(this)
        this.delay=5
 end,
    draw=function(this)
        this.delay-=1
        if this.delay<-30 then
            destroy_object(this)
        elseif this.delay<0 then
           
            rectfill(24,58,104,70,0)
            --rect(26,64-10,102,64+10,7)
            --print("---",31,64-2,13)
            if room.x==3 and room.y==1 then
                print("old site",48,62,7)
            elseif level_index()==30 then
                print("summit",52,62,7)
            else
                local level=(1+level_index())*100
                print(level.." m",52+(level<1000 and 2 or 0),62,7)
            end
            --print("---",86,64-2,13)
           
            draw_time(4,4)
        end
    end
}

-- object functions --
-----------------------

function init_object(type,x,y)
    if type.if_not_fruit~=nil and got_fruit[1+level_index()] then
        return
    end
    local obj = {}
    obj.type = type
    obj.collideable=true
    obj.solids=true

    obj.spr = type.tile
    obj.flip = {x=false,y=false}

    obj.x = x
    obj.y = y
    obj.hitbox = { x=0,y=0,w=8,h=8 }

    obj.spd = {x=0,y=0}
    obj.rem = {x=0,y=0}

    obj.is_solid=function(ox,oy)
        if oy>0 and not obj.check(platform,ox,0) and obj.check(platform,ox,oy) then
            return true
        end
        return solid_at(obj.x+obj.hitbox.x+ox,obj.y+obj.hitbox.y+oy,obj.hitbox.w,obj.hitbox.h)
         or obj.check(fall_floor,ox,oy)
         or obj.check(fake_wall,ox,oy)
    end
   
    obj.is_ice=function(ox,oy)
        return ice_at(obj.x+obj.hitbox.x+ox,obj.y+obj.hitbox.y+oy,obj.hitbox.w,obj.hitbox.h)
    end
   
    obj.collide=function(type,ox,oy)
        local other
        for i=1,count(objects) do
            other=objects[i]
            if other ~=nil and other.type == type and other != obj and other.collideable and
                other.x+other.hitbox.x+other.hitbox.w > obj.x+obj.hitbox.x+ox and 
                other.y+other.hitbox.y+other.hitbox.h > obj.y+obj.hitbox.y+oy and
                other.x+other.hitbox.x < obj.x+obj.hitbox.x+obj.hitbox.w+ox and 
                other.y+other.hitbox.y < obj.y+obj.hitbox.y+obj.hitbox.h+oy then
                return other
            end
        end
        return nil
    end
   
    obj.check=function(type,ox,oy)
        return obj.collide(type,ox,oy) ~=nil
    end
   
    obj.move=function(ox,oy)
        local amount
        -- [x] get move amount
     obj.rem.x += ox
        amount = flr(obj.rem.x + 0.5)
        obj.rem.x -= amount
        obj.move_x(amount,0)
       
        -- [y] get move amount
        obj.rem.y += oy
        amount = flr(obj.rem.y + 0.5)
        obj.rem.y -= amount
        obj.move_y(amount)
    end
   
    obj.move_x=function(amount,start)
        if obj.solids then
            local step = sign(amount)
            for i=start,abs(amount) do
                if not obj.is_solid(step,0) then
                    obj.x += step
                else
                    obj.spd.x = 0
                    obj.rem.x = 0
                    break
                end
            end
        else
            obj.x += amount
        end
    end
   
    obj.move_y=function(amount)
        if obj.solids then
            local step = sign(amount)
            for i=0,abs(amount) do
             if not obj.is_solid(0,step) then
                    obj.y += step
                else
                    obj.spd.y = 0
                    obj.rem.y = 0
                    break
                end
            end
        else
            obj.y += amount
        end
    end

    add(objects,obj)
    if obj.type.init~=nil then
        obj.type.init(obj)
    end
    return obj
end

function destroy_object(obj)
    del(objects,obj)
end

function kill_player(obj)
    sfx_timer=12
    sfx(0)
    deaths+=1
    shake=10
    destroy_object(obj)
    dead_particles={}
    for dir=0,7 do
        local angle=(dir/8)
        add(dead_particles,{
            x=obj.x+4,
            y=obj.y+4,
            t=10,
            spd={
                x=sin(angle)*3,
                y=cos(angle)*3
            }
        })
        restart_room()
    end
end

-- room functions --
--------------------

function restart_room()
    will_restart=true
    delay_restart=15
end

function next_room()
 if room.x==2 and room.y==1 then
  music(30,500,7)
 elseif room.x==3 and room.y==1 then
  music(20,500,7)
 elseif room.x==4 and room.y==2 then
  music(30,500,7)
 elseif room.x==5 and room.y==3 then
  music(30,500,7)
 end

    if room.x==7 then
        load_room(0,room.y+1)
    else
        load_room(room.x+1,room.y)
    end
end

function load_room(x,y)
    has_dashed=false
    has_key=false

    --remove existing objects
    foreach(objects,destroy_object)

    --current room
    room.x = x
    room.y = y

    -- entities
    for tx=0,15 do
        for ty=0,15 do
            local tile = mget(room.x*16+tx,room.y*16+ty);
            if tile==11 then
                init_object(platform,tx*8,ty*8).dir=-1
            elseif tile==12 then
                init_object(platform,tx*8,ty*8).dir=1
            else
                foreach(types, 
                function(type) 
                    if type.tile == tile then
                        init_object(type,tx*8,ty*8) 
                    end 
                end)
            end
        end
    end
   
    if not is_title() then
        init_object(room_title,0,0)
    end
end

-- update function --
-----------------------

function _update()
    frames=((frames+1)%30)
    if frames==0 and level_index()<30 then
        seconds=((seconds+1)%60)
        if seconds==0 then
            minutes+=1
        end
    end
   
    if music_timer>0 then
     music_timer-=1
     if music_timer<=0 then
      music(10,0,7)
     end
    end
   
    if sfx_timer>0 then
     sfx_timer-=1
    end
   
    -- cancel if freeze
    if freeze>0 then freeze-=1 return end

    -- screenshake
    if shake>0 then
        shake-=1
        camera()
        if shake>0 then
            camera(-2+rnd(5),-2+rnd(5))
        end
    end
   
    -- restart (soon)
    if will_restart and delay_restart>0 then
        delay_restart-=1
        if delay_restart<=0 then
            will_restart=false
            load_room(room.x,room.y)
        end
    end

    -- update each object
    foreach(objects,function(obj)
        obj.move(obj.spd.x,obj.spd.y)
        if obj.type.update~=nil then
            obj.type.update(obj) 
        end
    end)
   
    -- start game
    if is_title() then
        if not start_game and (btn(k_jump) or btn(k_dash)) then
            music(-1)
            start_game_flash=50
            start_game=true
            sfx(38)
        end
        if start_game then
            start_game_flash-=1
            if start_game_flash<=-30 then
                begin_game()
            end
        end
    end
end

-- drawing functions --
-----------------------
function _draw()
    if freeze>0 then return end
   
    -- reset all palette values
    pal()
   
    -- start game flash
    if start_game then
        local c=10
        if start_game_flash>10 then
            if frames%10<5 then
                c=7
            end
        elseif start_game_flash>5 then
            c=2
        elseif start_game_flash>0 then
            c=1
        else 
            c=0
        end
        if c<10 then
            pal(6,c)
            pal(12,c)
            pal(13,c)
            pal(5,c)
            pal(1,c)
            pal(7,c)
        end
    end

    -- clear screen
    local bg_col = 0
    if flash_bg then
        bg_col = frames/5
    elseif new_bg~=nil then
        bg_col=2
    end
    rectfill(0,0,128,128,bg_col)

    -- clouds
    if not is_title() then
        foreach(clouds, function(c)
            c.x += c.spd
            rectfill(c.x,c.y,c.x+c.w,c.y+4+(1-c.w/64)*12,new_bg~=nil and 14 or 1)
            if c.x > 128 then
                c.x = -c.w
                c.y=rnd(128-8)
            end
        end)
    end

    -- draw bg terrain
    map(room.x * 16,room.y * 16,0,0,16,16,4)

    -- platforms/big chest
    foreach(objects, function(o)
        if o.type==platform or o.type==big_chest then
            draw_object(o)
        end
    end)

    -- draw terrain
    local off=is_title() and -4 or 0
    map(room.x*16,room.y * 16,off,0,16,16,2)
   
    -- draw objects
    foreach(objects, function(o)
        if o.type~=platform and o.type~=big_chest then
            draw_object(o)
        end
    end)
   
    -- draw fg terrain
    map(room.x * 16,room.y * 16,0,0,16,16,8)
   
    -- particles
    foreach(particles, function(p)
        p.x += p.spd
        p.y += sin(p.off)
        p.off+= min(0.05,p.spd/32)
        rectfill(p.x,p.y,p.x+p.s,p.y+p.s,p.c)
        if p.x>128+4 then 
            p.x=-4
            p.y=rnd(128)
        end
    end)
   
    -- dead particles
    foreach(dead_particles, function(p)
        p.x += p.spd.x
        p.y += p.spd.y
        p.t -=1
        if p.t <= 0 then del(dead_particles,p) end
        rectfill(p.x-p.t/5,p.y-p.t/5,p.x+p.t/5,p.y+p.t/5,14+p.t%2)
    end)
   
    -- draw outside of the screen for screenshake
    rectfill(-5,-5,-1,133,0)
    rectfill(-5,-5,133,-1,0)
    rectfill(-5,128,133,133,0)
    rectfill(128,-5,133,133,0)
   
    -- credits
    if is_title() then
        print("x+c",58,80,5)
        print("matt thorson",42,96,5)
        print("noel berry",46,102,5)
    end
   
    if level_index()==30 then
        local p
        for i=1,count(objects) do
            if objects[i].type==player then
                p = objects[i]
                break
            end
        end
        if p~=nil then
            local diff=min(24,40-abs(p.x+4-64))
            rectfill(0,0,diff,128,0)
            rectfill(128-diff,0,128,128,0)
        end
    end

end

function draw_object(obj)

    if obj.type.draw ~=nil then
        obj.type.draw(obj)
    elseif obj.spr > 0 then
        spr(obj.spr,obj.x,obj.y,1,1,obj.flip.x,obj.flip.y)
    end

end

function draw_time(x,y)

    local s=seconds
    local m=minutes%60
    local h=flr(minutes/60)
   
    rectfill(x,y,x+32,y+6,0)
    print((h<10 and "0"..h or h)..":"..(m<10 and "0"..m or m)..":"..(s<10 and "0"..s or s),x+1,y+1,7)

end

-- helper functions --
----------------------

function clamp(val,a,b)
    return max(a, min(b, val))
end

function appr(val,target,amount)
 return val > target 
     and max(val - amount, target) 
     or min(val + amount, target)
end

function sign(v)
    return v>0 and 1 or
                                v<0 and -1 or 0
end

function maybe()
    return rnd(1)<0.5
end

function solid_at(x,y,w,h)
 return tile_flag_at(x,y,w,h,0)
end

function ice_at(x,y,w,h)
 return tile_flag_at(x,y,w,h,4)
end

function tile_flag_at(x,y,w,h,flag)
 for i=max(0,flr(x/8)),min(15,(x+w-1)/8) do
     for j=max(0,flr(y/8)),min(15,(y+h-1)/8) do
         if fget(tile_at(i,j),flag) then
             return true
         end
     end
 end
    return false
end

function tile_at(x,y)
 return mget(room.x * 16 + x, room.y * 16 + y)
end

function spikes_at(x,y,w,h,xspd,yspd)
 for i=max(0,flr(x/8)),min(15,(x+w-1)/8) do
     for j=max(0,flr(y/8)),min(15,(y+h-1)/8) do
      local tile=tile_at(i,j)
      if tile==17 and ((y+h-1)%8>=6 or y+h==j*8+8) and yspd>=0 then
       return true
      elseif tile==27 and y%8<=2 and yspd<=0 then
       return true
         elseif tile==43 and x%8<=2 and xspd<=0 then
          return true
         elseif tile==59 and ((x+w-1)%8>=6 or x+w==i*8+8) and xspd>=0 then
          return true
         end
     end
 end
    return false
end