#[doc = "Register `SS_ICS` reader"]
pub type R = crate::R<SsIcsSpec>;
#[doc = "Register `SS_ICS` writer"]
pub type W = crate::W<SsIcsSpec>;
#[doc = "Field `ICS` reader - 0:0\\]
This bit contains the External TimeStamp Counter Overflow Interrupt status. Write '1' to clear bits. (ICS - Interrupt Clear Shadow Register)"]
pub type IcsR = crate::BitReader;
#[doc = "Field `ICS` writer - 0:0\\]
This bit contains the External TimeStamp Counter Overflow Interrupt status. Write '1' to clear bits. (ICS - Interrupt Clear Shadow Register)"]
pub type IcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU2` reader - 31:1\\]
Reserved"]
pub type Nu2R = crate::FieldReader<u32>;
#[doc = "Field `NU2` writer - 31:1\\]
Reserved"]
pub type Nu2W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
This bit contains the External TimeStamp Counter Overflow Interrupt status. Write '1' to clear bits. (ICS - Interrupt Clear Shadow Register)"]
    #[inline(always)]
    pub fn ics(&self) -> IcsR {
        IcsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn nu2(&self) -> Nu2R {
        Nu2R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
This bit contains the External TimeStamp Counter Overflow Interrupt status. Write '1' to clear bits. (ICS - Interrupt Clear Shadow Register)"]
    #[inline(always)]
    #[must_use]
    pub fn ics(&mut self) -> IcsW<SsIcsSpec> {
        IcsW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu2(&mut self) -> Nu2W<SsIcsSpec> {
        Nu2W::new(self, 1)
    }
}
#[doc = "SS_ICS\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ics::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ics::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsIcsSpec;
impl crate::RegisterSpec for SsIcsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_ics::R`](R) reader structure"]
impl crate::Readable for SsIcsSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_ics::W`](W) writer structure"]
impl crate::Writable for SsIcsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_ICS to value 0"]
impl crate::Resettable for SsIcsSpec {
    const RESET_VALUE: u32 = 0;
}
