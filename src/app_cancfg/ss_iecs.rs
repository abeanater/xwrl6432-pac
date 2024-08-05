#[doc = "Register `SS_IECS` reader"]
pub type R = crate::R<SsIecsSpec>;
#[doc = "Register `SS_IECS` writer"]
pub type W = crate::W<SsIecsSpec>;
#[doc = "Field `IECS` reader - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Write '1' to clear bits. (IECS - Interrupt Enable Clear Shadow Register)"]
pub type IecsR = crate::BitReader;
#[doc = "Field `IECS` writer - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Write '1' to clear bits. (IECS - Interrupt Enable Clear Shadow Register)"]
pub type IecsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU4` reader - 31:1\\]
Reserved"]
pub type Nu4R = crate::FieldReader<u32>;
#[doc = "Field `NU4` writer - 31:1\\]
Reserved"]
pub type Nu4W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Write '1' to clear bits. (IECS - Interrupt Enable Clear Shadow Register)"]
    #[inline(always)]
    pub fn iecs(&self) -> IecsR {
        IecsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn nu4(&self) -> Nu4R {
        Nu4R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Write '1' to clear bits. (IECS - Interrupt Enable Clear Shadow Register)"]
    #[inline(always)]
    #[must_use]
    pub fn iecs(&mut self) -> IecsW<SsIecsSpec> {
        IecsW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu4(&mut self) -> Nu4W<SsIecsSpec> {
        Nu4W::new(self, 1)
    }
}
#[doc = "SS_IECS\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_iecs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_iecs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsIecsSpec;
impl crate::RegisterSpec for SsIecsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_iecs::R`](R) reader structure"]
impl crate::Readable for SsIecsSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_iecs::W`](W) writer structure"]
impl crate::Writable for SsIecsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_IECS to value 0"]
impl crate::Resettable for SsIecsSpec {
    const RESET_VALUE: u32 = 0;
}
