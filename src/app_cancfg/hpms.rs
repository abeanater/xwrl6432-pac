#[doc = "Register `HPMS` reader"]
pub type R = crate::R<HpmsSpec>;
#[doc = "Register `HPMS` writer"]
pub type W = crate::W<HpmsSpec>;
#[doc = "Field `BIDX` reader - 5:0\\]
Buffer Index"]
pub type BidxR = crate::FieldReader;
#[doc = "Field `BIDX` writer - 5:0\\]
Buffer Index"]
pub type BidxW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `MSI` reader - 7:6\\]
Message Storeage Indicator"]
pub type MsiR = crate::FieldReader;
#[doc = "Field `MSI` writer - 7:6\\]
Message Storeage Indicator"]
pub type MsiW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIDX` reader - 14:8\\]
Filter Index"]
pub type FidxR = crate::FieldReader;
#[doc = "Field `FIDX` writer - 14:8\\]
Filter Index"]
pub type FidxW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `FLST` reader - 15:15\\]
Filter List"]
pub type FlstR = crate::BitReader;
#[doc = "Field `FLST` writer - 15:15\\]
Filter List"]
pub type FlstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU40` reader - 31:16\\]
Reserved"]
pub type Nu40R = crate::FieldReader<u16>;
#[doc = "Field `NU40` writer - 31:16\\]
Reserved"]
pub type Nu40W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Buffer Index"]
    #[inline(always)]
    pub fn bidx(&self) -> BidxR {
        BidxR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Message Storeage Indicator"]
    #[inline(always)]
    pub fn msi(&self) -> MsiR {
        MsiR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Filter Index"]
    #[inline(always)]
    pub fn fidx(&self) -> FidxR {
        FidxR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - 15:15\\]
Filter List"]
    #[inline(always)]
    pub fn flst(&self) -> FlstR {
        FlstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    pub fn nu40(&self) -> Nu40R {
        Nu40R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Buffer Index"]
    #[inline(always)]
    #[must_use]
    pub fn bidx(&mut self) -> BidxW<HpmsSpec> {
        BidxW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Message Storeage Indicator"]
    #[inline(always)]
    #[must_use]
    pub fn msi(&mut self) -> MsiW<HpmsSpec> {
        MsiW::new(self, 6)
    }
    #[doc = "Bits 8:14 - 14:8\\]
Filter Index"]
    #[inline(always)]
    #[must_use]
    pub fn fidx(&mut self) -> FidxW<HpmsSpec> {
        FidxW::new(self, 8)
    }
    #[doc = "Bit 15 - 15:15\\]
Filter List"]
    #[inline(always)]
    #[must_use]
    pub fn flst(&mut self) -> FlstW<HpmsSpec> {
        FlstW::new(self, 15)
    }
    #[doc = "Bits 16:31 - 31:16\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu40(&mut self) -> Nu40W<HpmsSpec> {
        Nu40W::new(self, 16)
    }
}
#[doc = "HPMS\n\nYou can [`read`](crate::Reg::read) this register and get [`hpms::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpms::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HpmsSpec;
impl crate::RegisterSpec for HpmsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpms::R`](R) reader structure"]
impl crate::Readable for HpmsSpec {}
#[doc = "`write(|w| ..)` method takes [`hpms::W`](W) writer structure"]
impl crate::Writable for HpmsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPMS to value 0"]
impl crate::Resettable for HpmsSpec {
    const RESET_VALUE: u32 = 0;
}
