#[doc = "Register `APPSS_OWRITE_ERR_AGGR` reader"]
pub type R = crate::R<AppssOwriteErrAggrSpec>;
#[doc = "Register `APPSS_OWRITE_ERR_AGGR` writer"]
pub type W = crate::W<AppssOwriteErrAggrSpec>;
#[doc = "Field `err` reader - 0:0\\]
Ored error of all write error signals -Sticky Bit"]
pub type ErrR = crate::BitReader;
#[doc = "Field `err` writer - 0:0\\]
Ored error of all write error signals -Sticky Bit"]
pub type ErrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Ored error of all write error signals -Sticky Bit"]
    #[inline(always)]
    pub fn err(&self) -> ErrR {
        ErrR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Ored error of all write error signals -Sticky Bit"]
    #[inline(always)]
    #[must_use]
    pub fn err(&mut self) -> ErrW<AppssOwriteErrAggrSpec> {
        ErrW::new(self, 0)
    }
}
#[doc = "APPSS_OWRITE_ERR_AGGR\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_owrite_err_aggr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_owrite_err_aggr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssOwriteErrAggrSpec;
impl crate::RegisterSpec for AppssOwriteErrAggrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_owrite_err_aggr::R`](R) reader structure"]
impl crate::Readable for AppssOwriteErrAggrSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_owrite_err_aggr::W`](W) writer structure"]
impl crate::Writable for AppssOwriteErrAggrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_OWRITE_ERR_AGGR to value 0"]
impl crate::Resettable for AppssOwriteErrAggrSpec {
    const RESET_VALUE: u32 = 0;
}
