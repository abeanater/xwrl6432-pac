#[doc = "Register `HWACCREG1` reader"]
pub type R = crate::R<Hwaccreg1Spec>;
#[doc = "Register `HWACCREG1` writer"]
pub type W = crate::W<Hwaccreg1Spec>;
#[doc = "Field `ACCENABLE` reader - 2:0\\]
Enable/Disable Control: A value of ACC_ENABLE = 111b enables the Radar Hardware Accelerator and any other value of the register keeps the Accelerator Engine in disabled state."]
pub type AccenableR = crate::FieldReader;
#[doc = "Field `ACCENABLE` writer - 2:0\\]
Enable/Disable Control: A value of ACC_ENABLE = 111b enables the Radar Hardware Accelerator and any other value of the register keeps the Accelerator Engine in disabled state."]
pub type AccenableW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ACCCLKEN` reader - 5:3\\]
Clock-gating Control: This register bit controls the enable/disable for the clock of the Radar Accelerator. This register bit can be set to 0 to clock-gate the accelerator when not using the accelerator. Before enabling the accelerator or before configuring the registers of accelerator, this register bit should be set to 111b first, so that the clock is available."]
pub type AccclkenR = crate::FieldReader;
#[doc = "Field `ACCCLKEN` writer - 5:3\\]
Clock-gating Control: This register bit controls the enable/disable for the clock of the Radar Accelerator. This register bit can be set to 0 to clock-gate the accelerator when not using the accelerator. Before enabling the accelerator or before configuring the registers of accelerator, this register bit should be set to 111b first, so that the clock is available."]
pub type AccclkenW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ACCRESET` reader - 8:6\\]
Software Reset Control: This register provides software reset control for the Radar Hardware Accelerator. The assertion of these register bits by the main processor will bring the Accelerator Engine to a known reset state. This is mostly applicable for resetting the accelerator in case of unexpected behavior. The sequence to be followed in case software reset is to write 111b to this register and then a 000b"]
pub type AccresetR = crate::FieldReader;
#[doc = "Field `ACCRESET` writer - 8:6\\]
Software Reset Control: This register provides software reset control for the Radar Hardware Accelerator. The assertion of these register bits by the main processor will bring the Accelerator Engine to a known reset state. This is mostly applicable for resetting the accelerator in case of unexpected behavior. The sequence to be followed in case software reset is to write 111b to this register and then a 000b"]
pub type AccresetW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::BitReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FFT1DEN` reader - 10:10\\]
ADC buffer sharing mode This register is relevant where the Radar Hardware Accelerator is included in a single device along with the mmWave RF front-end. In such a case, during active chirp transmission and inline 1st dimension FFT processing, the ACCEL_MEM0 and ACCEL_MEM1 memories of the accelerator are shared as ping-pong ADC buffers. This register bit needs to be set during this time, so that while the Digital Front End writes ADC samples to the ping buffer, the accelerator automatically accesses (only) the pong buffer, and vice versa. At the end of the active transmission portion of a frame, this bit can be cleared, so that the accelerator has access to all the four local memories independently."]
pub type Fft1denR = crate::BitReader;
#[doc = "Field `FFT1DEN` writer - 10:10\\]
ADC buffer sharing mode This register is relevant where the Radar Hardware Accelerator is included in a single device along with the mmWave RF front-end. In such a case, during active chirp transmission and inline 1st dimension FFT processing, the ACCEL_MEM0 and ACCEL_MEM1 memories of the accelerator are shared as ping-pong ADC buffers. This register bit needs to be set during this time, so that while the Digital Front End writes ADC samples to the ping buffer, the accelerator automatically accesses (only) the pong buffer, and vice versa. At the end of the active transmission portion of a frame, this bit can be cleared, so that the accelerator has access to all the four local memories independently."]
pub type Fft1denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCDYNCLKEN` reader - 11:11\\]
Dynamic Clock-gating Control:Setting this register bit to 1 enables the capability to clock gate the Radar Accelerator core IPs (FFT and CFAR-CA datapath,CFAR-OS datapath, memory compression datapath) based on the ParamSet being executed."]
pub type AccdynclkenR = crate::BitReader;
#[doc = "Field `ACCDYNCLKEN` writer - 11:11\\]
Dynamic Clock-gating Control:Setting this register bit to 1 enables the capability to clock gate the Radar Accelerator core IPs (FFT and CFAR-CA datapath,CFAR-OS datapath, memory compression datapath) based on the ParamSet being executed."]
pub type AccdynclkenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACCDYNCLKEN_LEVEL2` reader - 12:12\\]
Level 2 dynamic clock-gating control :- Setting this register bit to 1 will lead to further power saving by disabling clock during FSM wait state."]
pub type AccdynclkenLevel2R = crate::BitReader;
#[doc = "Field `ACCDYNCLKEN_LEVEL2` writer - 12:12\\]
Level 2 dynamic clock-gating control :- Setting this register bit to 1 will lead to further power saving by disabling clock during FSM wait state."]
pub type AccdynclkenLevel2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - "]
pub type Nu2R = crate::FieldReader<u32>;
#[doc = "Field `NU2` writer - "]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 19, u32>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Enable/Disable Control: A value of ACC_ENABLE = 111b enables the Radar Hardware Accelerator and any other value of the register keeps the Accelerator Engine in disabled state."]
    #[inline(always)]
    pub fn accenable(&self) -> AccenableR {
        AccenableR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Clock-gating Control: This register bit controls the enable/disable for the clock of the Radar Accelerator. This register bit can be set to 0 to clock-gate the accelerator when not using the accelerator. Before enabling the accelerator or before configuring the registers of accelerator, this register bit should be set to 111b first, so that the clock is available."]
    #[inline(always)]
    pub fn accclken(&self) -> AccclkenR {
        AccclkenR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Software Reset Control: This register provides software reset control for the Radar Hardware Accelerator. The assertion of these register bits by the main processor will bring the Accelerator Engine to a known reset state. This is mostly applicable for resetting the accelerator in case of unexpected behavior. The sequence to be followed in case software reset is to write 111b to this register and then a 000b"]
    #[inline(always)]
    pub fn accreset(&self) -> AccresetR {
        AccresetR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 10:10\\]
ADC buffer sharing mode This register is relevant where the Radar Hardware Accelerator is included in a single device along with the mmWave RF front-end. In such a case, during active chirp transmission and inline 1st dimension FFT processing, the ACCEL_MEM0 and ACCEL_MEM1 memories of the accelerator are shared as ping-pong ADC buffers. This register bit needs to be set during this time, so that while the Digital Front End writes ADC samples to the ping buffer, the accelerator automatically accesses (only) the pong buffer, and vice versa. At the end of the active transmission portion of a frame, this bit can be cleared, so that the accelerator has access to all the four local memories independently."]
    #[inline(always)]
    pub fn fft1den(&self) -> Fft1denR {
        Fft1denR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 11:11\\]
Dynamic Clock-gating Control:Setting this register bit to 1 enables the capability to clock gate the Radar Accelerator core IPs (FFT and CFAR-CA datapath,CFAR-OS datapath, memory compression datapath) based on the ParamSet being executed."]
    #[inline(always)]
    pub fn accdynclken(&self) -> AccdynclkenR {
        AccdynclkenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 12:12\\]
Level 2 dynamic clock-gating control :- Setting this register bit to 1 will lead to further power saving by disabling clock during FSM wait state."]
    #[inline(always)]
    pub fn accdynclken_level2(&self) -> AccdynclkenLevel2R {
        AccdynclkenLevel2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:31"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new((self.bits >> 13) & 0x0007_ffff)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Enable/Disable Control: A value of ACC_ENABLE = 111b enables the Radar Hardware Accelerator and any other value of the register keeps the Accelerator Engine in disabled state."]
    #[inline(always)]
    #[must_use]
    pub fn accenable(&mut self) -> AccenableW<Hwaccreg1Spec> {
        AccenableW::new(self, 0)
    }
    #[doc = "Bits 3:5 - 5:3\\]
Clock-gating Control: This register bit controls the enable/disable for the clock of the Radar Accelerator. This register bit can be set to 0 to clock-gate the accelerator when not using the accelerator. Before enabling the accelerator or before configuring the registers of accelerator, this register bit should be set to 111b first, so that the clock is available."]
    #[inline(always)]
    #[must_use]
    pub fn accclken(&mut self) -> AccclkenW<Hwaccreg1Spec> {
        AccclkenW::new(self, 3)
    }
    #[doc = "Bits 6:8 - 8:6\\]
Software Reset Control: This register provides software reset control for the Radar Hardware Accelerator. The assertion of these register bits by the main processor will bring the Accelerator Engine to a known reset state. This is mostly applicable for resetting the accelerator in case of unexpected behavior. The sequence to be followed in case software reset is to write 111b to this register and then a 000b"]
    #[inline(always)]
    #[must_use]
    pub fn accreset(&mut self) -> AccresetW<Hwaccreg1Spec> {
        AccresetW::new(self, 6)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<Hwaccreg1Spec> {
        Nu1W::new(self, 9)
    }
    #[doc = "Bit 10 - 10:10\\]
ADC buffer sharing mode This register is relevant where the Radar Hardware Accelerator is included in a single device along with the mmWave RF front-end. In such a case, during active chirp transmission and inline 1st dimension FFT processing, the ACCEL_MEM0 and ACCEL_MEM1 memories of the accelerator are shared as ping-pong ADC buffers. This register bit needs to be set during this time, so that while the Digital Front End writes ADC samples to the ping buffer, the accelerator automatically accesses (only) the pong buffer, and vice versa. At the end of the active transmission portion of a frame, this bit can be cleared, so that the accelerator has access to all the four local memories independently."]
    #[inline(always)]
    #[must_use]
    pub fn fft1den(&mut self) -> Fft1denW<Hwaccreg1Spec> {
        Fft1denW::new(self, 10)
    }
    #[doc = "Bit 11 - 11:11\\]
Dynamic Clock-gating Control:Setting this register bit to 1 enables the capability to clock gate the Radar Accelerator core IPs (FFT and CFAR-CA datapath,CFAR-OS datapath, memory compression datapath) based on the ParamSet being executed."]
    #[inline(always)]
    #[must_use]
    pub fn accdynclken(&mut self) -> AccdynclkenW<Hwaccreg1Spec> {
        AccdynclkenW::new(self, 11)
    }
    #[doc = "Bit 12 - 12:12\\]
Level 2 dynamic clock-gating control :- Setting this register bit to 1 will lead to further power saving by disabling clock during FSM wait state."]
    #[inline(always)]
    #[must_use]
    pub fn accdynclken_level2(&mut self) -> AccdynclkenLevel2W<Hwaccreg1Spec> {
        AccdynclkenLevel2W::new(self, 12)
    }
    #[doc = "Bits 13:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<Hwaccreg1Spec> {
        Nu2W::new(self, 13)
    }
}
#[doc = "HWACCREG1\n\nYou can [`read`](crate::Reg::read) this register and get [`hwaccreg1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hwaccreg1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hwaccreg1Spec;
impl crate::RegisterSpec for Hwaccreg1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwaccreg1::R`](R) reader structure"]
impl crate::Readable for Hwaccreg1Spec {}
#[doc = "`write(|w| ..)` method takes [`hwaccreg1::W`](W) writer structure"]
impl crate::Writable for Hwaccreg1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWACCREG1 to value 0"]
impl crate::Resettable for Hwaccreg1Spec {
    const RESET_VALUE: u32 = 0;
}
