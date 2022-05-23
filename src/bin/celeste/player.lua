player = {
    init = function(this)
        this.p_jump = false
        this.p_dash = false
        this.grace = 0
        this.jbuffer = 0
        this.djump = max_djump
        this.dash_time = 0
        this.dash_effect_time = 0
        this.dash_target = {
            x = 0,
            y = 0
        }
        this.dash_accel = {
            x = 0,
            y = 0
        }
        this.hitbox = {
            x = 1,
            y = 3,
            w = 6,
            h = 5
        }
        this.spr_off = 0
        this.was_on_ground = false
        create_hair(this)
    end,
    update=function(this)
        if (pause_player) then
            return
        end
       
        local input = btn(k_right) and 1 or (btn(k_left) and -1 or 0)
       
        -- spikes collide
        if spikes_at(this.x+this.hitbox.x,this.y+this.hitbox.y,this.hitbox.w,this.hitbox.h,this.spd.x,this.spd.y) then
            kill_player(this)
        end
         
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
            this.jbuffer = 4;
        elseif this.jbuffer>0 then
            this.jbuffer -= 1;
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

        this.dash_effect_time -=1;
  
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
            if this.spd.x != 0 then
                this.flip.x=(this.spd.x<0)
            end

            -- gravity
            local maxfall=2
            local gravity=0.21

    if abs(this.spd.y) <= 0.15 then
        gravity *= 0.5;
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
                    if wall_dir != 0 then
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
   
    draw = function(this)

        -- clamp in screen
        if this.x < -1 or this.x > 121 then
            this.x = clamp(this.x, -1, 121)
            this.spd.x = 0
        end

        set_hair_color(this.djump)
        draw_hair(this, this.flip.x and -1 or 1)
        spr(this.spr, this.x, this.y, 1, 1, this.flip.x, this.flip.y)
        unset_hair_color()
    end
}
