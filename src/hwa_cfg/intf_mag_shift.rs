#[doc = "Register `INTF_MAG_SHIFT` reader"]
pub type R = crate::R<IntfMagShiftSpec>;
#[doc = "Register `INTF_MAG_SHIFT` writer"]
pub type W = crate::W<IntfMagShiftSpec>;
#[doc = "Field `INTF_MAG_SHIFT` reader - 3:0\\]
Right shift applied after scaling ΓÇô 2^(4+INTERFSUM_MAG_SHIFT). Can t be more than 2^(17)."]
pub type IntfMagShiftR = crate::FieldReader;
#[doc = "Field `INTF_MAG_SHIFT` writer - 3:0\\]
Right shift applied after scaling ΓÇô 2^(4+INTERFSUM_MAG_SHIFT). Can t be more than 2^(17)."]
pub type IntfMagShiftW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 28, u32>;
impl R {
    #[doc = "Bits 0:3 - 3:0\\]
Right shift applied after scaling ΓÇô 2^(4+INTERFSUM_MAG_SHIFT). Can t be more than 2^(17)."]
    #[inline(always)]
    pub fn intf_mag_shift(&self) -> IntfMagShiftR {
        IntfMagShiftR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 4) & 0x0fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:3 - 3:0\\]
Right shift applied after scaling ΓÇô 2^(4+INTERFSUM_MAG_SHIFT). Can t be more than 2^(17)."]
    #[inline(always)]
    #[must_use]
    pub fn intf_mag_shift(&mut self) -> IntfMagShiftW<IntfMagShiftSpec> {
        IntfMagShiftW::new(self, 0)
    }
    #[doc = "Bits 4:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagShiftSpec> {
        Nu1W::new(self, 4)
    }
}
#[doc = "INTF_MAG_SHIFT\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_mag_shift::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_mag_shift::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagShiftSpec;
impl crate::RegisterSpec for IntfMagShiftSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_mag_shift::R`](R) reader structure"]
impl crate::Readable for IntfMagShiftSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_mag_shift::W`](W) writer structure"]
impl crate::Writable for IntfMagShiftSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAG_SHIFT to value 0"]
impl crate::Resettable for IntfMagShiftSpec {
    const RESET_VALUE: u32 = 0;
}
