#[doc = "Register `DED_EOI_REG` reader"]
pub type R = crate::R<DedEoiRegSpec>;
#[doc = "Register `DED_EOI_REG` writer"]
pub type W = crate::W<DedEoiRegSpec>;
#[doc = "Field `DED_EOI_WR` reader - 0:0\\]
EOI Register. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field. This bit is self clearing, reading this bit will return 0."]
pub type DedEoiWrR = crate::BitReader;
#[doc = "Field `DED_EOI_WR` writer - 0:0\\]
EOI Register. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field. This bit is self clearing, reading this bit will return 0."]
pub type DedEoiWrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU11` reader - 31:1\\]
Reserved"]
pub type Nu11R = crate::FieldReader<u32>;
#[doc = "Field `NU11` writer - 31:1\\]
Reserved"]
pub type Nu11W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
EOI Register. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field. This bit is self clearing, reading this bit will return 0."]
    #[inline(always)]
    pub fn ded_eoi_wr(&self) -> DedEoiWrR {
        DedEoiWrR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn nu11(&self) -> Nu11R {
        Nu11R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
EOI Register. Writing 1 to any bit will set the corresponding bit. Reads do not alter the value of the field. This bit is self clearing, reading this bit will return 0."]
    #[inline(always)]
    #[must_use]
    pub fn ded_eoi_wr(&mut self) -> DedEoiWrW<DedEoiRegSpec> {
        DedEoiWrW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu11(&mut self) -> Nu11W<DedEoiRegSpec> {
        Nu11W::new(self, 1)
    }
}
#[doc = "EOI Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ded_eoi_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ded_eoi_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DedEoiRegSpec;
impl crate::RegisterSpec for DedEoiRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ded_eoi_reg::R`](R) reader structure"]
impl crate::Readable for DedEoiRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ded_eoi_reg::W`](W) writer structure"]
impl crate::Writable for DedEoiRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DED_EOI_REG to value 0"]
impl crate::Resettable for DedEoiRegSpec {
    const RESET_VALUE: u32 = 0;
}
