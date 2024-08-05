#[doc = "Register `TDCR` reader"]
pub type R = crate::R<TdcrSpec>;
#[doc = "Register `TDCR` writer"]
pub type W = crate::W<TdcrSpec>;
#[doc = "Field `TDCF` reader - 6:0\\]
Transmitter Delay Compensation Filter Window Length"]
pub type TdcfR = crate::FieldReader;
#[doc = "Field `TDCF` writer - 6:0\\]
Transmitter Delay Compensation Filter Window Length"]
pub type TdcfW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU28` reader - 7:7\\]
Reserved"]
pub type Nu28R = crate::BitReader;
#[doc = "Field `NU28` writer - 7:7\\]
Reserved"]
pub type Nu28W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCO` reader - 14:8\\]
Transmitter Delay Compensation Offset"]
pub type TdcoR = crate::FieldReader;
#[doc = "Field `TDCO` writer - 14:8\\]
Transmitter Delay Compensation Offset"]
pub type TdcoW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `NU29` reader - 31:15\\]
Reserved"]
pub type Nu29R = crate::FieldReader<u32>;
#[doc = "Field `NU29` writer - 31:15\\]
Reserved"]
pub type Nu29W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:6 - 6:0\\]
Transmitter Delay Compensation Filter Window Length"]
    #[inline(always)]
    pub fn tdcf(&self) -> TdcfR {
        TdcfR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved"]
    #[inline(always)]
    pub fn nu28(&self) -> Nu28R {
        Nu28R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Transmitter Delay Compensation Offset"]
    #[inline(always)]
    pub fn tdco(&self) -> TdcoR {
        TdcoR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Reserved"]
    #[inline(always)]
    pub fn nu29(&self) -> Nu29R {
        Nu29R::new((self.bits >> 15) & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:6 - 6:0\\]
Transmitter Delay Compensation Filter Window Length"]
    #[inline(always)]
    #[must_use]
    pub fn tdcf(&mut self) -> TdcfW<TdcrSpec> {
        TdcfW::new(self, 0)
    }
    #[doc = "Bit 7 - 7:7\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu28(&mut self) -> Nu28W<TdcrSpec> {
        Nu28W::new(self, 7)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Transmitter Delay Compensation Offset"]
    #[inline(always)]
    #[must_use]
    pub fn tdco(&mut self) -> TdcoW<TdcrSpec> {
        TdcoW::new(self, 8)
    }
    #[doc = "Bits 15:31 - 31:15\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu29(&mut self) -> Nu29W<TdcrSpec> {
        Nu29W::new(self, 15)
    }
}
#[doc = "TDCR\n\nYou can [`read`](crate::Reg::read) this register and get [`tdcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdcrSpec;
impl crate::RegisterSpec for TdcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tdcr::R`](R) reader structure"]
impl crate::Readable for TdcrSpec {}
#[doc = "`write(|w| ..)` method takes [`tdcr::W`](W) writer structure"]
impl crate::Writable for TdcrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TDCR to value 0"]
impl crate::Resettable for TdcrSpec {
    const RESET_VALUE: u32 = 0;
}
