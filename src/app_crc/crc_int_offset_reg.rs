#[doc = "Register `CRC_INT_OFFSET_REG` reader"]
pub type R = crate::R<CrcIntOffsetRegSpec>;
#[doc = "Register `CRC_INT_OFFSET_REG` writer"]
pub type W = crate::W<CrcIntOffsetRegSpec>;
#[doc = "Field `OFSTREG` reader - 7:0\\]
CRC Interrupt Offset. This register indicates the highest priority pending interrupt vector address. Reading the offset register auto- matically clear the respective interrupt flag. Please reference Table 1ΓÇô3. for details."]
pub type OfstregR = crate::FieldReader;
#[doc = "Field `OFSTREG` writer - 7:0\\]
CRC Interrupt Offset. This register indicates the highest priority pending interrupt vector address. Reading the offset register auto- matically clear the respective interrupt flag. Please reference Table 1ΓÇô3. for details."]
pub type OfstregW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `Reserved1` reader - 31:8\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
#[doc = "Field `Reserved1` writer - 31:8\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
CRC Interrupt Offset. This register indicates the highest priority pending interrupt vector address. Reading the offset register auto- matically clear the respective interrupt flag. Please reference Table 1ΓÇô3. for details."]
    #[inline(always)]
    pub fn ofstreg(&self) -> OfstregR {
        OfstregR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
CRC Interrupt Offset. This register indicates the highest priority pending interrupt vector address. Reading the offset register auto- matically clear the respective interrupt flag. Please reference Table 1ΓÇô3. for details."]
    #[inline(always)]
    #[must_use]
    pub fn ofstreg(&mut self) -> OfstregW<CrcIntOffsetRegSpec> {
        OfstregW::new(self, 0)
    }
    #[doc = "Bits 8:31 - 31:8\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcIntOffsetRegSpec> {
        Reserved1W::new(self, 8)
    }
}
#[doc = "Contains the interrupt offset vector address\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_int_offset_reg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_int_offset_reg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcIntOffsetRegSpec;
impl crate::RegisterSpec for CrcIntOffsetRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_int_offset_reg::R`](R) reader structure"]
impl crate::Readable for CrcIntOffsetRegSpec {}
#[doc = "`write(|w| ..)` method takes [`crc_int_offset_reg::W`](W) writer structure"]
impl crate::Writable for CrcIntOffsetRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_INT_OFFSET_REG to value 0"]
impl crate::Resettable for CrcIntOffsetRegSpec {
    const RESET_VALUE: u32 = 0;
}
