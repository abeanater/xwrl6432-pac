#[doc = "Register `INTF_MAGTHRESH6` reader"]
pub type R = crate::R<IntfMagthresh6Spec>;
#[doc = "Register `INTF_MAGTHRESH6` writer"]
pub type W = crate::W<IntfMagthresh6Spec>;
#[doc = "Field `INTF_MAGTHRESH6` reader - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =5"]
pub type IntfMagthresh6R = crate::FieldReader<u32>;
#[doc = "Field `INTF_MAGTHRESH6` writer - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =5"]
pub type IntfMagthresh6W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =5"]
    #[inline(always)]
    pub fn intf_magthresh6(&self) -> IntfMagthresh6R {
        IntfMagthresh6R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude threshold by interference statistics for bcnt =5"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magthresh6(&mut self) -> IntfMagthresh6W<IntfMagthresh6Spec> {
        IntfMagthresh6W::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagthresh6Spec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "INTF_MAGTHRESH6\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magthresh6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magthresh6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagthresh6Spec;
impl crate::RegisterSpec for IntfMagthresh6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magthresh6::R`](R) reader structure"]
impl crate::Readable for IntfMagthresh6Spec {}
#[doc = "`write(|w| ..)` method takes [`intf_magthresh6::W`](W) writer structure"]
impl crate::Writable for IntfMagthresh6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGTHRESH6 to value 0"]
impl crate::Resettable for IntfMagthresh6Spec {
    const RESET_VALUE: u32 = 0;
}
