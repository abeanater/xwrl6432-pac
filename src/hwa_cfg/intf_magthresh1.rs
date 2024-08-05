#[doc = "Register `INTF_MAGTHRESH1` reader"]
pub type R = crate::R<IntfMagthresh1Spec>;
#[doc = "Register `INTF_MAGTHRESH1` writer"]
pub type W = crate::W<IntfMagthresh1Spec>;
#[doc = "Field `INTF_MAGTHRESH1` reader - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =0"]
pub type IntfMagthresh1R = crate::FieldReader<u32>;
#[doc = "Field `INTF_MAGTHRESH1` writer - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =0"]
pub type IntfMagthresh1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =0"]
    #[inline(always)]
    pub fn intf_magthresh1(&self) -> IntfMagthresh1R {
        IntfMagthresh1R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magthresh1(&mut self) -> IntfMagthresh1W<IntfMagthresh1Spec> {
        IntfMagthresh1W::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagthresh1Spec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "INTF_MAGTHRESH1\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagthresh1Spec;
impl crate::RegisterSpec for IntfMagthresh1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magthresh1::R`](R) reader structure"]
impl crate::Readable for IntfMagthresh1Spec {}
#[doc = "`write(|w| ..)` method takes [`intf_magthresh1::W`](W) writer structure"]
impl crate::Writable for IntfMagthresh1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGTHRESH1 to value 0"]
impl crate::Resettable for IntfMagthresh1Spec {
    const RESET_VALUE: u32 = 0;
}
