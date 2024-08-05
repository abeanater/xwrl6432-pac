#[doc = "Register `SS_IES` reader"]
pub type R = crate::R<SsIesSpec>;
#[doc = "Register `SS_IES` writer"]
pub type W = crate::W<SsIesSpec>;
#[doc = "Field `IES` reader - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Read Enabled Interrupts. (IES - Interrupt Enable Status)"]
pub type IesR = crate::BitReader;
#[doc = "Field `IES` writer - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Read Enabled Interrupts. (IES - Interrupt Enable Status)"]
pub type IesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU6` reader - 31:1\\]
Reserved"]
pub type Nu6R = crate::FieldReader<u32>;
#[doc = "Field `NU6` writer - 31:1\\]
Reserved"]
pub type Nu6W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Read Enabled Interrupts. (IES - Interrupt Enable Status)"]
    #[inline(always)]
    pub fn ies(&self) -> IesR {
        IesR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn nu6(&self) -> Nu6R {
        Nu6R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Read Enabled Interrupts. (IES - Interrupt Enable Status)"]
    #[inline(always)]
    #[must_use]
    pub fn ies(&mut self) -> IesW<SsIesSpec> {
        IesW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu6(&mut self) -> Nu6W<SsIesSpec> {
        Nu6W::new(self, 1)
    }
}
#[doc = "SS_IES\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ies::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ies::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsIesSpec;
impl crate::RegisterSpec for SsIesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_ies::R`](R) reader structure"]
impl crate::Readable for SsIesSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_ies::W`](W) writer structure"]
impl crate::Writable for SsIesSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_IES to value 0"]
impl crate::Resettable for SsIesSpec {
    const RESET_VALUE: u32 = 0;
}
