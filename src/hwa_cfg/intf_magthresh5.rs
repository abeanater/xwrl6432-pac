#[doc = "Register `INTF_MAGTHRESH5` reader"]
pub type R = crate::R<IntfMagthresh5Spec>;
#[doc = "Register `INTF_MAGTHRESH5` writer"]
pub type W = crate::W<IntfMagthresh5Spec>;
#[doc = "Field `INTF_MAGTHRESH5` reader - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =4"]
pub type IntfMagthresh5R = crate::FieldReader<u32>;
#[doc = "Field `INTF_MAGTHRESH5` writer - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =4"]
pub type IntfMagthresh5W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =4"]
    #[inline(always)]
    pub fn intf_magthresh5(&self) -> IntfMagthresh5R {
        IntfMagthresh5R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =4"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magthresh5(&mut self) -> IntfMagthresh5W<IntfMagthresh5Spec> {
        IntfMagthresh5W::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagthresh5Spec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "INTF_MAGTHRESH5\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagthresh5Spec;
impl crate::RegisterSpec for IntfMagthresh5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magthresh5::R`](R) reader structure"]
impl crate::Readable for IntfMagthresh5Spec {}
#[doc = "`write(|w| ..)` method takes [`intf_magthresh5::W`](W) writer structure"]
impl crate::Writable for IntfMagthresh5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGTHRESH5 to value 0"]
impl crate::Resettable for IntfMagthresh5Spec {
    const RESET_VALUE: u32 = 0;
}
