#[doc = "Register `INTF_MAGTHRESH3` reader"]
pub type R = crate::R<IntfMagthresh3Spec>;
#[doc = "Register `INTF_MAGTHRESH3` writer"]
pub type W = crate::W<IntfMagthresh3Spec>;
#[doc = "Field `INTF_MAGTHRESH3` reader - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =2"]
pub type IntfMagthresh3R = crate::FieldReader<u32>;
#[doc = "Field `INTF_MAGTHRESH3` writer - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =2"]
pub type IntfMagthresh3W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =2"]
    #[inline(always)]
    pub fn intf_magthresh3(&self) -> IntfMagthresh3R {
        IntfMagthresh3R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =2"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magthresh3(&mut self) -> IntfMagthresh3W<IntfMagthresh3Spec> {
        IntfMagthresh3W::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagthresh3Spec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "INTF_MAGTHRESH3\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagthresh3Spec;
impl crate::RegisterSpec for IntfMagthresh3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magthresh3::R`](R) reader structure"]
impl crate::Readable for IntfMagthresh3Spec {}
#[doc = "`write(|w| ..)` method takes [`intf_magthresh3::W`](W) writer structure"]
impl crate::Writable for IntfMagthresh3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGTHRESH3 to value 0"]
impl crate::Resettable for IntfMagthresh3Spec {
    const RESET_VALUE: u32 = 0;
}
