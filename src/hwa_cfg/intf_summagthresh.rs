#[doc = "Register `INTF_SUMMAGTHRESH` reader"]
pub type R = crate::R<IntfSummagthreshSpec>;
#[doc = "Register `INTF_SUMMAGTHRESH` writer"]
pub type W = crate::W<IntfSummagthreshSpec>;
#[doc = "Field `INTF_SUMMAGTHRESH` reader - 23:0\\]
Indicates the sum of mag values ; only Configured BCNT mag values are added"]
pub type IntfSummagthreshR = crate::FieldReader<u32>;
#[doc = "Field `INTF_SUMMAGTHRESH` writer - 23:0\\]
Indicates the sum of mag values ; only Configured BCNT mag values are added"]
pub type IntfSummagthreshW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates the sum of mag values ; only Configured BCNT mag values are added"]
    #[inline(always)]
    pub fn intf_summagthresh(&self) -> IntfSummagthreshR {
        IntfSummagthreshR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates the sum of mag values ; only Configured BCNT mag values are added"]
    #[inline(always)]
    #[must_use]
    pub fn intf_summagthresh(&mut self) -> IntfSummagthreshW<IntfSummagthreshSpec> {
        IntfSummagthreshW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfSummagthreshSpec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "INTF_SUMMAGTHRESH\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_summagthresh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_summagthresh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfSummagthreshSpec;
impl crate::RegisterSpec for IntfSummagthreshSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_summagthresh::R`](R) reader structure"]
impl crate::Readable for IntfSummagthreshSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_summagthresh::W`](W) writer structure"]
impl crate::Writable for IntfSummagthreshSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_SUMMAGTHRESH to value 0"]
impl crate::Resettable for IntfSummagthreshSpec {
    const RESET_VALUE: u32 = 0;
}
