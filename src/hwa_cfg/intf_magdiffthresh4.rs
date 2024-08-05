#[doc = "Register `INTF_MAGDIFFTHRESH4` reader"]
pub type R = crate::R<IntfMagdiffthresh4Spec>;
#[doc = "Register `INTF_MAGDIFFTHRESH4` writer"]
pub type W = crate::W<IntfMagdiffthresh4Spec>;
#[doc = "Field `INTF_MAGDIFFTHRESH4` reader - 23:0\\]
Indicates interference magnitude difference threshold by interference statistics for bcnt =3"]
pub type IntfMagdiffthresh4R = crate::FieldReader<u32>;
#[doc = "Field `INTF_MAGDIFFTHRESH4` writer - 23:0\\]
Indicates interference magnitude difference threshold by interference statistics for bcnt =3"]
pub type IntfMagdiffthresh4W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude difference threshold by interference statistics for bcnt =3"]
    #[inline(always)]
    pub fn intf_magdiffthresh4(&self) -> IntfMagdiffthresh4R {
        IntfMagdiffthresh4R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
Indicates interference magnitude difference threshold by interference statistics for bcnt =3"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magdiffthresh4(&mut self) -> IntfMagdiffthresh4W<IntfMagdiffthresh4Spec> {
        IntfMagdiffthresh4W::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagdiffthresh4Spec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "INTF_MAGDIFFTHRESH4\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagdiffthresh4Spec;
impl crate::RegisterSpec for IntfMagdiffthresh4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magdiffthresh4::R`](R) reader structure"]
impl crate::Readable for IntfMagdiffthresh4Spec {}
#[doc = "`write(|w| ..)` method takes [`intf_magdiffthresh4::W`](W) writer structure"]
impl crate::Writable for IntfMagdiffthresh4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGDIFFTHRESH4 to value 0"]
impl crate::Resettable for IntfMagdiffthresh4Spec {
    const RESET_VALUE: u32 = 0;
}
