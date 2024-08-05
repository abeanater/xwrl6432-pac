#[doc = "Register `SCIFORMAT` reader"]
pub type R = crate::R<SciformatSpec>;
#[doc = "Register `SCIFORMAT` writer"]
pub type W = crate::W<SciformatSpec>;
#[doc = "Field `CHAR` reader - 2:0\\]
Character length control bits. These bits are effective in SCI compatible mode only. These bits set the SCI character length from 1 to 8 bits. Note: In compatibility mode or buffered SCI mode, when data of fewer than eight bits in length is received, it is left justified in SCIRD/RDy and padded with trailing zeros. Data read from the SCIRD should be shifted by software to make the received data right justified. Note: Data written to the SCITD should be right justified but does not need to be padded with leading zeros. These bits are witable in SCI mode only."]
pub type CharR = crate::FieldReader;
#[doc = "Field `CHAR` writer - 2:0\\]
Character length control bits. These bits are effective in SCI compatible mode only. These bits set the SCI character length from 1 to 8 bits. Note: In compatibility mode or buffered SCI mode, when data of fewer than eight bits in length is received, it is left justified in SCIRD/RDy and padded with trailing zeros. Data read from the SCIRD should be shifted by software to make the received data right justified. Note: Data written to the SCITD should be right justified but does not need to be padded with leading zeros. These bits are witable in SCI mode only."]
pub type CharW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LENGTH` reader - 18:16\\]
Frame length control bits. In LIN mode, these bits indicate the number of bytes in the response field from 1 to 8 bytes. In buffered SCI mode, these bits indicate the number of characters. When these bits are used to indicate LIN response length (SCIGCR1\\[0\\]
= 1), then when there is an ID RX match, this value should be updated with the expected length of the response. In buffered SCI mode, these bits indicate the number of characters with SCIFORMAT\\[2:0\\]
bits per character. i.e. these bits indicate the transmitter/receiver format for the number of characters: 1 to 8. There can be up to eight characters with eight bits each."]
pub type LengthR = crate::FieldReader;
#[doc = "Field `LENGTH` writer - 18:16\\]
Frame length control bits. In LIN mode, these bits indicate the number of bytes in the response field from 1 to 8 bytes. In buffered SCI mode, these bits indicate the number of characters. When these bits are used to indicate LIN response length (SCIGCR1\\[0\\]
= 1), then when there is an ID RX match, this value should be updated with the expected length of the response. In buffered SCI mode, these bits indicate the number of characters with SCIFORMAT\\[2:0\\]
bits per character. i.e. these bits indicate the transmitter/receiver format for the number of characters: 1 to 8. There can be up to eight characters with eight bits each."]
pub type LengthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - 31:19\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:19\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:2 - 2:0\\]
Character length control bits. These bits are effective in SCI compatible mode only. These bits set the SCI character length from 1 to 8 bits. Note: In compatibility mode or buffered SCI mode, when data of fewer than eight bits in length is received, it is left justified in SCIRD/RDy and padded with trailing zeros. Data read from the SCIRD should be shifted by software to make the received data right justified. Note: Data written to the SCITD should be right justified but does not need to be padded with leading zeros. These bits are witable in SCI mode only."]
    #[inline(always)]
    pub fn char(&self) -> CharR {
        CharR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Frame length control bits. In LIN mode, these bits indicate the number of bytes in the response field from 1 to 8 bytes. In buffered SCI mode, these bits indicate the number of characters. When these bits are used to indicate LIN response length (SCIGCR1\\[0\\]
= 1), then when there is an ID RX match, this value should be updated with the expected length of the response. In buffered SCI mode, these bits indicate the number of characters with SCIFORMAT\\[2:0\\]
bits per character. i.e. these bits indicate the transmitter/receiver format for the number of characters: 1 to 8. There can be up to eight characters with eight bits each."]
    #[inline(always)]
    pub fn length(&self) -> LengthR {
        LengthR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 19) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:2 - 2:0\\]
Character length control bits. These bits are effective in SCI compatible mode only. These bits set the SCI character length from 1 to 8 bits. Note: In compatibility mode or buffered SCI mode, when data of fewer than eight bits in length is received, it is left justified in SCIRD/RDy and padded with trailing zeros. Data read from the SCIRD should be shifted by software to make the received data right justified. Note: Data written to the SCITD should be right justified but does not need to be padded with leading zeros. These bits are witable in SCI mode only."]
    #[inline(always)]
    #[must_use]
    pub fn char(&mut self) -> CharW<SciformatSpec> {
        CharW::new(self, 0)
    }
    #[doc = "Bits 16:18 - 18:16\\]
Frame length control bits. In LIN mode, these bits indicate the number of bytes in the response field from 1 to 8 bytes. In buffered SCI mode, these bits indicate the number of characters. When these bits are used to indicate LIN response length (SCIGCR1\\[0\\]
= 1), then when there is an ID RX match, this value should be updated with the expected length of the response. In buffered SCI mode, these bits indicate the number of characters with SCIFORMAT\\[2:0\\]
bits per character. i.e. these bits indicate the transmitter/receiver format for the number of characters: 1 to 8. There can be up to eight characters with eight bits each."]
    #[inline(always)]
    #[must_use]
    pub fn length(&mut self) -> LengthW<SciformatSpec> {
        LengthW::new(self, 16)
    }
    #[doc = "Bits 19:31 - 31:19\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<SciformatSpec> {
        Reserved1W::new(self, 19)
    }
}
#[doc = "The SCIFORMAT register is used to set up the character and frame lengths.\n\nYou can [`read`](crate::Reg::read) this register and get [`sciformat::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sciformat::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SciformatSpec;
impl crate::RegisterSpec for SciformatSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sciformat::R`](R) reader structure"]
impl crate::Readable for SciformatSpec {}
#[doc = "`write(|w| ..)` method takes [`sciformat::W`](W) writer structure"]
impl crate::Writable for SciformatSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SCIFORMAT to value 0"]
impl crate::Resettable for SciformatSpec {
    const RESET_VALUE: u32 = 0;
}
