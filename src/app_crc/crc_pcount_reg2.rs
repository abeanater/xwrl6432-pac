#[doc = "Register `CRC_PCOUNT_REG2` reader"]
pub type R = crate::R<CrcPcountReg2Spec>;
#[doc = "Register `CRC_PCOUNT_REG2` writer"]
pub type W = crate::W<CrcPcountReg2Spec>;
#[doc = "Field `CRC_PAT_COUNT2` reader - 19:0\\]
Channel 2 Pattern Counter Preload Register. This register con- tains the number of data patterns in one sector to be compressed before a CRC is performed."]
pub type CrcPatCount2R = crate::FieldReader<u32>;
#[doc = "Field `CRC_PAT_COUNT2` writer - 19:0\\]
Channel 2 Pattern Counter Preload Register. This register con- tains the number of data patterns in one sector to be compressed before a CRC is performed."]
pub type CrcPatCount2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `Reserved1` reader - 31:20\\]
Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved1` writer - 31:20\\]
Reserved"]
pub type Reserved1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:19 - 19:0\\]
Channel 2 Pattern Counter Preload Register. This register con- tains the number of data patterns in one sector to be compressed before a CRC is performed."]
    #[inline(always)]
    pub fn crc_pat_count2(&self) -> CrcPatCount2R {
        CrcPatCount2R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:19 - 19:0\\]
Channel 2 Pattern Counter Preload Register. This register con- tains the number of data patterns in one sector to be compressed before a CRC is performed."]
    #[inline(always)]
    #[must_use]
    pub fn crc_pat_count2(&mut self) -> CrcPatCount2W<CrcPcountReg2Spec> {
        CrcPatCount2W::new(self, 0)
    }
    #[doc = "Bits 20:31 - 31:20\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn reserved1(&mut self) -> Reserved1W<CrcPcountReg2Spec> {
        Reserved1W::new(self, 20)
    }
}
#[doc = "Channel 2 preload register for the pattern count\n\nYou can [`read`](crate::Reg::read) this register and get [`crc_pcount_reg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crc_pcount_reg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrcPcountReg2Spec;
impl crate::RegisterSpec for CrcPcountReg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crc_pcount_reg2::R`](R) reader structure"]
impl crate::Readable for CrcPcountReg2Spec {}
#[doc = "`write(|w| ..)` method takes [`crc_pcount_reg2::W`](W) writer structure"]
impl crate::Writable for CrcPcountReg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRC_PCOUNT_REG2 to value 0"]
impl crate::Resettable for CrcPcountReg2Spec {
    const RESET_VALUE: u32 = 0;
}
