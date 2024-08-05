#[doc = "Register `RTITBLCOMP` reader"]
pub type R = crate::R<RtitblcompSpec>;
#[doc = "Register `RTITBLCOMP` writer"]
pub type W = crate::W<RtitblcompSpec>;
#[doc = "Field `TBLCOMP` reader - 31:0\\]
TBLCOMP: Timebase Low Compare Value. This value determines when the edge detection circuit starts monitoring the NTUx signal. It will be compared with Up Counter 0. User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
pub type TblcompR = crate::FieldReader<u32>;
#[doc = "Field `TBLCOMP` writer - 31:0\\]
TBLCOMP: Timebase Low Compare Value. This value determines when the edge detection circuit starts monitoring the NTUx signal. It will be compared with Up Counter 0. User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
pub type TblcompW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
TBLCOMP: Timebase Low Compare Value. This value determines when the edge detection circuit starts monitoring the NTUx signal. It will be compared with Up Counter 0. User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
    #[inline(always)]
    pub fn tblcomp(&self) -> TblcompR {
        TblcompR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
TBLCOMP: Timebase Low Compare Value. This value determines when the edge detection circuit starts monitoring the NTUx signal. It will be compared with Up Counter 0. User and privilege mode (read): current compare value Privilege mode (write when TBEXT = 0): the compare value is updated Privilege mode (write when TBEXT = 1): the compare value is not changed Note: Reset behavior A reset does not generate a compare match."]
    #[inline(always)]
    #[must_use]
    pub fn tblcomp(&mut self) -> TblcompW<RtitblcompSpec> {
        TblcompW::new(self, 0)
    }
}
#[doc = "Timebase Low Compare compare value to activate edge detection circuit\n\nYou can [`read`](crate::Reg::read) this register and get [`rtitblcomp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtitblcomp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtitblcompSpec;
impl crate::RegisterSpec for RtitblcompSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtitblcomp::R`](R) reader structure"]
impl crate::Readable for RtitblcompSpec {}
#[doc = "`write(|w| ..)` method takes [`rtitblcomp::W`](W) writer structure"]
impl crate::Writable for RtitblcompSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RTITBLCOMP to value 0"]
impl crate::Resettable for RtitblcompSpec {
    const RESET_VALUE: u32 = 0;
}
