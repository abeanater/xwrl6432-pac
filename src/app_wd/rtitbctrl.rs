#[doc = "Register `RTITBCTRL` reader"]
pub type R = crate::R<RtitbctrlSpec>;
#[doc = "Register `RTITBCTRL` writer"]
pub type W = crate::W<RtitbctrlSpec>;
#[doc = "Field `TBEXT` reader - 0:0\\]
TBEXT: Timebase External. The Timebase External bit selects whether the Free Running Counter 0 is clocked by the internal Up Counter 0 or from the external signal NTUx. Since setting the TBEXT bit to 1 resets Up Counter 0, Free Running Counter 0 will not be incremented in this occurence. The only source which is able to increment Free Running Counter 0 is NTUx. When the Timebase Supervisor circuit detects a missing clockedge, then the TBEXT bit is reset. The selection if the external signal should be used, can only be done by software. User and privilege mode (read): 0 = UC0 clocks FRC0 1 = NTUx clocks FRC0 Privilege mode (write): 0 = MUX is switched to internal UC0 clocking scheme 1 = MUX is switched to external NTUx clocking scheme"]
pub type TbextR = crate::BitReader;
#[doc = "Field `TBEXT` writer - 0:0\\]
TBEXT: Timebase External. The Timebase External bit selects whether the Free Running Counter 0 is clocked by the internal Up Counter 0 or from the external signal NTUx. Since setting the TBEXT bit to 1 resets Up Counter 0, Free Running Counter 0 will not be incremented in this occurence. The only source which is able to increment Free Running Counter 0 is NTUx. When the Timebase Supervisor circuit detects a missing clockedge, then the TBEXT bit is reset. The selection if the external signal should be used, can only be done by software. User and privilege mode (read): 0 = UC0 clocks FRC0 1 = NTUx clocks FRC0 Privilege mode (write): 0 = MUX is switched to internal UC0 clocking scheme 1 = MUX is switched to external NTUx clocking scheme"]
pub type TbextW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INC` reader - 1:1\\]
INC: Increment Free Running Counter 0. This bit determines wether the Free Running Counter 0 is automatically incremented if a failing clock on the NTUx signal is detected. User and privilege mode (read): 0 = FRC0 will not be incremented 1 = FRC0 will be incremented Privilege mode (write): 0 = Do not increment FRC0 on failing external clock 1 = Increment FRC0 on failing external clock"]
pub type IncR = crate::BitReader;
#[doc = "Field `INC` writer - 1:1\\]
INC: Increment Free Running Counter 0. This bit determines wether the Free Running Counter 0 is automatically incremented if a failing clock on the NTUx signal is detected. User and privilege mode (read): 0 = FRC0 will not be incremented 1 = FRC0 will be incremented Privilege mode (write): 0 = Do not increment FRC0 on failing external clock 1 = Increment FRC0 on failing external clock"]
pub type IncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESERVED3` reader - 31:2\\]
Reserved"]
pub type Reserved3R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED3` writer - 31:2\\]
Reserved"]
pub type Reserved3W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
TBEXT: Timebase External. The Timebase External bit selects whether the Free Running Counter 0 is clocked by the internal Up Counter 0 or from the external signal NTUx. Since setting the TBEXT bit to 1 resets Up Counter 0, Free Running Counter 0 will not be incremented in this occurence. The only source which is able to increment Free Running Counter 0 is NTUx. When the Timebase Supervisor circuit detects a missing clockedge, then the TBEXT bit is reset. The selection if the external signal should be used, can only be done by software. User and privilege mode (read): 0 = UC0 clocks FRC0 1 = NTUx clocks FRC0 Privilege mode (write): 0 = MUX is switched to internal UC0 clocking scheme 1 = MUX is switched to external NTUx clocking scheme"]
    #[inline(always)]
    pub fn tbext(&self) -> TbextR {
        TbextR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
INC: Increment Free Running Counter 0. This bit determines wether the Free Running Counter 0 is automatically incremented if a failing clock on the NTUx signal is detected. User and privilege mode (read): 0 = FRC0 will not be incremented 1 = FRC0 will be incremented Privilege mode (write): 0 = Do not increment FRC0 on failing external clock 1 = Increment FRC0 on failing external clock"]
    #[inline(always)]
    pub fn inc(&self) -> IncR {
        IncR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
TBEXT: Timebase External. The Timebase External bit selects whether the Free Running Counter 0 is clocked by the internal Up Counter 0 or from the external signal NTUx. Since setting the TBEXT bit to 1 resets Up Counter 0, Free Running Counter 0 will not be incremented in this occurence. The only source which is able to increment Free Running Counter 0 is NTUx. When the Timebase Supervisor circuit detects a missing clockedge, then the TBEXT bit is reset. The selection if the external signal should be used, can only be done by software. User and privilege mode (read): 0 = UC0 clocks FRC0 1 = NTUx clocks FRC0 Privilege mode (write): 0 = MUX is switched to internal UC0 clocking scheme 1 = MUX is switched to external NTUx clocking scheme"]
    #[inline(always)]
    #[must_use]
    pub fn tbext(&mut self) -> TbextW<RtitbctrlSpec> {
        TbextW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
INC: Increment Free Running Counter 0. This bit determines wether the Free Running Counter 0 is automatically incremented if a failing clock on the NTUx signal is detected. User and privilege mode (read): 0 = FRC0 will not be incremented 1 = FRC0 will be incremented Privilege mode (write): 0 = Do not increment FRC0 on failing external clock 1 = Increment FRC0 on failing external clock"]
    #[inline(always)]
    #[must_use]
    pub fn inc(&mut self) -> IncW<RtitbctrlSpec> {
        IncW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved3(&mut self) -> Reserved3W<RtitbctrlSpec> {
        Reserved3W::new(self, 2)
    }
}
#[doc = "Timebase Control selection which source triggers free running counter 0\n\nYou can [`read`](crate::Reg::read) this register and get [`rtitbctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtitbctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtitbctrlSpec;
impl crate::RegisterSpec for RtitbctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtitbctrl::R`](R) reader structure"]
impl crate::Readable for RtitbctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`rtitbctrl::W`](W) writer structure"]
impl crate::Writable for RtitbctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTITBCTRL to value 0"]
impl crate::Resettable for RtitbctrlSpec {
    const RESET_VALUE: u32 = 0;
}
