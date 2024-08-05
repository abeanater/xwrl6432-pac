#[doc = "Register `INTF_MAGDIFFTHRESH6` reader"]
pub type R = crate::R<IntfMagdiffthresh6Spec>;
#[doc = "Register `INTF_MAGDIFFTHRESH6` writer"]
pub type W = crate::W<IntfMagdiffthresh6Spec>;
#[doc = "Field `INTF_MAGDIFFTHRESH6` reader - 23:0\\]
Indicates interference magnitude difference threshold by interference statistics for bcnt =5"]
pub type IntfMagdiffthresh6R = crate::FieldReader<u32>;
#[doc = "Field `INTF_MAGDIFFTHRESH6` writer - 23:0\\]
Indicates interference magnitude difference threshold by interference statistics for bcnt =5"]
pub type IntfMagdiffthresh6W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude difference threshold by interference statistics for bcnt =5"]
    #[inline(always)]
    pub fn intf_magdiffthresh6(&self) -> IntfMagdiffthresh6R {
        IntfMagdiffthresh6R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude difference threshold by interference statistics for bcnt =5"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magdiffthresh6(&mut self) -> IntfMagdiffthresh6W<IntfMagdiffthresh6Spec> {
        IntfMagdiffthresh6W::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagdiffthresh6Spec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "INTF_MAGDIFFTHRESH6\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh6::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh6::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagdiffthresh6Spec;
impl crate::RegisterSpec for IntfMagdiffthresh6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magdiffthresh6::R`](R) reader structure"]
impl crate::Readable for IntfMagdiffthresh6Spec {}
#[doc = "`write(|w| ..)` method takes [`intf_magdiffthresh6::W`](W) writer structure"]
impl crate::Writable for IntfMagdiffthresh6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGDIFFTHRESH6 to value 0"]
impl crate::Resettable for IntfMagdiffthresh6Spec {
    const RESET_VALUE: u32 = 0;
}
