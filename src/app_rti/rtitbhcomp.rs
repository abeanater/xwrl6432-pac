#[doc = "Register `RTITBHCOMP` reader"]
pub type R = crate::R<RtitbhcompSpec>;
#[doc = "Register `RTITBHCOMP` writer"]
pub type W = crate::W<RtitbhcompSpec>;
#[doc = "Field `TBHCOMP` reader - 31:0\\]
TBHCOMP: Timebase High Compare Value. This value determines when the edge detection circuit will stop monitoring the NTUx signal. It will be compared with Up Counter 0. RTITBHCOMP has to be less than RTICPUC0, since RTIUC0 will be reset when RTICPUC0 is reached. Example: The NTUx edge detection circuit should be active +/- 10 RTICLK cycles around RTICPUC0. RTICPUC0 = 0x00000050 RTITBLCOMP = 0x000046 RTITBHCOMP = 0x00000009 User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
pub type TbhcompR = crate::FieldReader<u32>;
#[doc = "Field `TBHCOMP` writer - 31:0\\]
TBHCOMP: Timebase High Compare Value. This value determines when the edge detection circuit will stop monitoring the NTUx signal. It will be compared with Up Counter 0. RTITBHCOMP has to be less than RTICPUC0, since RTIUC0 will be reset when RTICPUC0 is reached. Example: The NTUx edge detection circuit should be active +/- 10 RTICLK cycles around RTICPUC0. RTICPUC0 = 0x00000050 RTITBLCOMP = 0x000046 RTITBHCOMP = 0x00000009 User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
pub type TbhcompW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TBHCOMP: Timebase High Compare Value. This value determines when the edge detection circuit will stop monitoring the NTUx signal. It will be compared with Up Counter 0. RTITBHCOMP has to be less than RTICPUC0, since RTIUC0 will be reset when RTICPUC0 is reached. Example: The NTUx edge detection circuit should be active +/- 10 RTICLK cycles around RTICPUC0. RTICPUC0 = 0x00000050 RTITBLCOMP = 0x000046 RTITBHCOMP = 0x00000009 User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
    #[inline(always)]
    pub fn tbhcomp(&self) -> TbhcompR {
        TbhcompR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TBHCOMP: Timebase High Compare Value. This value determines when the edge detection circuit will stop monitoring the NTUx signal. It will be compared with Up Counter 0. RTITBHCOMP has to be less than RTICPUC0, since RTIUC0 will be reset when RTICPUC0 is reached. Example: The NTUx edge detection circuit should be active +/- 10 RTICLK cycles around RTICPUC0. RTICPUC0 = 0x00000050 RTITBLCOMP = 0x000046 RTITBHCOMP = 0x00000009 User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
    #[inline(always)]
    #[must_use]
    pub fn tbhcomp(&mut self) -> TbhcompW<RtitbhcompSpec> {
        TbhcompW::new(self, 0)
    }
}
#[doc = "Timebase High Compare compare value to deactivate edge detection circuit\n\nYou can [`read`](crate::Reg::read) this register and get [`rtitbhcomp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtitbhcomp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtitbhcompSpec;
impl crate::RegisterSpec for RtitbhcompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtitbhcomp::R`](R) reader structure"]
impl crate::Readable for RtitbhcompSpec {}
#[doc = "`write(|w| ..)` method takes [`rtitbhcomp::W`](W) writer structure"]
impl crate::Writable for RtitbhcompSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTITBHCOMP to value 0"]
impl crate::Resettable for RtitbhcompSpec {
    const RESET_VALUE: u32 = 0;
}
