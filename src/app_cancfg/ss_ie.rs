#[doc = "Register `SS_IE` reader"]
pub type R = crate::R<SsIeSpec>;
#[doc = "Register `SS_IE` writer"]
pub type W = crate::W<SsIeSpec>;
#[doc = "Field `IE` reader - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Write '1' to set interrupt enable. Read returns interrupt enable. (IE - Interrupt Enable Register)"]
pub type IeR = crate::BitReader;
#[doc = "Field `IE` writer - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Write '1' to set interrupt enable. Read returns interrupt enable. (IE - Interrupt Enable Register)"]
pub type IeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NU5` reader - 31:1\\]
Reserved"]
pub type Nu5R = crate::FieldReader<u32>;
#[doc = "Field `NU5` writer - 31:1\\]
Reserved"]
pub type Nu5W<'a, REG> = crate::FieldWriter<'a, REG, 31, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Write '1' to set interrupt enable. Read returns interrupt enable. (IE - Interrupt Enable Register)"]
    #[inline(always)]
    pub fn ie(&self) -> IeR {
        IeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    pub fn nu5(&self) -> Nu5R {
        Nu5R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
External TimeStamp Counter Overflow Interrupt. Write '1' to set interrupt enable. Read returns interrupt enable. (IE - Interrupt Enable Register)"]
    #[inline(always)]
    #[must_use]
    pub fn ie(&mut self) -> IeW<SsIeSpec> {
        IeW::new(self, 0)
    }
    #[doc = "Bits 1:31 - 31:1\\]
Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn nu5(&mut self) -> Nu5W<SsIeSpec> {
        Nu5W::new(self, 1)
    }
}
#[doc = "SS_IE\n\nYou can [`read`](crate::Reg::read) this register and get [`ss_ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ss_ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SsIeSpec;
impl crate::RegisterSpec for SsIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ss_ie::R`](R) reader structure"]
impl crate::Readable for SsIeSpec {}
#[doc = "`write(|w| ..)` method takes [`ss_ie::W`](W) writer structure"]
impl crate::Writable for SsIeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SS_IE to value 0"]
impl crate::Resettable for SsIeSpec {
    const RESET_VALUE: u32 = 0;
}
