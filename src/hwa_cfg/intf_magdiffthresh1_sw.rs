#[doc = "Register `INTF_MAGDIFFTHRESH1_SW` reader"]
pub type R = crate::R<IntfMagdiffthresh1SwSpec>;
#[doc = "Register `INTF_MAGDIFFTHRESH1_SW` writer"]
pub type W = crate::W<IntfMagdiffthresh1SwSpec>;
#[doc = "Field `INTF_MAGDIFFTHRESH1_SW` reader - 23:0\\]
This register provides software programmed interference magnitude difference threshold value for bcnt =0"]
pub type IntfMagdiffthresh1SwR = crate::FieldReader<u32>;
#[doc = "Field `INTF_MAGDIFFTHRESH1_SW` writer - 23:0\\]
This register provides software programmed interference magnitude difference threshold value for bcnt =0"]
pub type IntfMagdiffthresh1SwW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:23 - 23:0\\]
This register provides software programmed interference magnitude difference threshold value for bcnt =0"]
    #[inline(always)]
    pub fn intf_magdiffthresh1_sw(&self) -> IntfMagdiffthresh1SwR {
        IntfMagdiffthresh1SwR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - 23:0\\]
This register provides software programmed interference magnitude difference threshold value for bcnt =0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magdiffthresh1_sw(&mut self) -> IntfMagdiffthresh1SwW<IntfMagdiffthresh1SwSpec> {
        IntfMagdiffthresh1SwW::new(self, 0)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagdiffthresh1SwSpec> {
        Nu1W::new(self, 24)
    }
}
#[doc = "INTF_MAGDIFFTHRESH1_SW\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffthresh1_sw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffthresh1_sw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagdiffthresh1SwSpec;
impl crate::RegisterSpec for IntfMagdiffthresh1SwSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magdiffthresh1_sw::R`](R) reader structure"]
impl crate::Readable for IntfMagdiffthresh1SwSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_magdiffthresh1_sw::W`](W) writer structure"]
impl crate::Writable for IntfMagdiffthresh1SwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGDIFFTHRESH1_SW to value 0"]
impl crate::Resettable for IntfMagdiffthresh1SwSpec {
    const RESET_VALUE: u32 = 0;
}
