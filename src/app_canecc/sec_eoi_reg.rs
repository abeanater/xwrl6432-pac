#[doc = "Register `SEC_EOI_REG` reader"]
pub type R = crate::R<SecEoiRegSpec>;
#[doc = "Register `SEC_EOI_REG` writer"]
pub type W = crate::W<SecEoiRegSpec>;
#[doc = "Field `SEC_EOI_WR` reader - 0:0\\]
EOI Register. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field. This bit is self clearing, reading this bit will return 0."]
pub type SecEoiWrR = crate::BitReader;
#[doc = "Field `SEC_EOI_WR` writer - 0:0\\]
EOI Register. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field. This bit is self clearing, reading this bit will return 0."]
pub type SecEoiWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU7` reader - 31:1\\]
Reserved"]
pub type Nu7R = crate::FieldReader<u32>;
#[doc = "Field `NU7` writer - 31:1\\]
Reserved"]
pub type Nu7W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EOI Register. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field. This bit is self clearing, reading this bit will return 0."]
    #[inline(always)]
    pub fn sec_eoi_wr(&self) -> SecEoiWrR {
        SecEoiWrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn nu7(&self) -> Nu7R {
        Nu7R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
EOI Register. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field. This bit is self clearing, reading this bit will return 0."]
    #[inline(always)]
    #[must_use]
    pub fn sec_eoi_wr(&mut self) -> SecEoiWrW<SecEoiRegSpec> {
        SecEoiWrW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu7(&mut self) -> Nu7W<SecEoiRegSpec> {
        Nu7W::new(self, 1)
    }
}
#[doc = "EOI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sec_eoi_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sec_eoi_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SecEoiRegSpec;
impl crate::RegisterSpec for SecEoiRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sec_eoi_reg::R`](R) reader structure"]
impl crate::Readable for SecEoiRegSpec {}
#[doc = "`write(|w| ..)` method takes [`sec_eoi_reg::W`](W) writer structure"]
impl crate::Writable for SecEoiRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SEC_EOI_REG to value 0"]
impl crate::Resettable for SecEoiRegSpec {
    const RESET_VALUE: u32 = 0;
}
