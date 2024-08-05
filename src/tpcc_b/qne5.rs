#[doc = "Register `QNE5` reader"]
pub type R = crate::R<Qne5Spec>;
#[doc = "Register `QNE5` writer"]
pub type W = crate::W<Qne5Spec>;
#[doc = "Field `ENUM` reader - 5:0\\]
Event Number: Specifies the specific Event Number for the given entry in the Event Queue. For DMA Channel events (ER/ESR/CER) ENUM will range between 0 and NUM_DMACH (up to 63). For QDMA Channel events (QER) ENUM will range between 0 and NUM_QDMACH (up to 7)."]
pub type EnumR = crate::FieldReader;
#[doc = "Field `ENUM` writer - 5:0\\]
Event Number: Specifies the specific Event Number for the given entry in the Event Queue. For DMA Channel events (ER/ESR/CER) ENUM will range between 0 and NUM_DMACH (up to 63). For QDMA Channel events (QER) ENUM will range between 0 and NUM_QDMACH (up to 7)."]
pub type EnumW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `ETYPE` reader - 7:6\\]
Event Type: Specifies the specific Event Type for the given entry in the Event Queue."]
pub type EtypeR = crate::FieldReader;
#[doc = "Field `ETYPE` writer - 7:6\\]
Event Type: Specifies the specific Event Type for the given entry in the Event Queue."]
pub type EtypeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES44` reader - 31:8\\]
RESERVE FIELD"]
pub type Res44R = crate::FieldReader<u32>;
#[doc = "Field `RES44` writer - 31:8\\]
RESERVE FIELD"]
pub type Res44W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
Event Number: Specifies the specific Event Number for the given entry in the Event Queue. For DMA Channel events (ER/ESR/CER) ENUM will range between 0 and NUM_DMACH (up to 63). For QDMA Channel events (QER) ENUM will range between 0 and NUM_QDMACH (up to 7)."]
    #[inline(always)]
    pub fn enum_(&self) -> EnumR {
        EnumR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Event Type: Specifies the specific Event Type for the given entry in the Event Queue."]
    #[inline(always)]
    pub fn etype(&self) -> EtypeR {
        EtypeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - 31:8\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res44(&self) -> Res44R {
        Res44R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
Event Number: Specifies the specific Event Number for the given entry in the Event Queue. For DMA Channel events (ER/ESR/CER) ENUM will range between 0 and NUM_DMACH (up to 63). For QDMA Channel events (QER) ENUM will range between 0 and NUM_QDMACH (up to 7)."]
    #[inline(always)]
    #[must_use]
    pub fn enum_(&mut self) -> EnumW<Qne5Spec> {
        EnumW::new(self, 0)
    }
    #[doc = "Bits 6:7 - 7:6\\]
Event Type: Specifies the specific Event Type for the given entry in the Event Queue."]
    #[inline(always)]
    #[must_use]
    pub fn etype(&mut self) -> EtypeW<Qne5Spec> {
        EtypeW::new(self, 6)
    }
    #[doc = "Bits 8:31 - 31:8\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res44(&mut self) -> Res44W<Qne5Spec> {
        Res44W::new(self, 8)
    }
}
#[doc = "Event Queue Entry Diagram for Queue n - Entry 5\n\nYou can [`read`](crate::Reg::read) this register and get [`qne5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qne5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Qne5Spec;
impl crate::RegisterSpec for Qne5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qne5::R`](R) reader structure"]
impl crate::Readable for Qne5Spec {}
#[doc = "`write(|w| ..)` method takes [`qne5::W`](W) writer structure"]
impl crate::Writable for Qne5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QNE5 to value 0"]
impl crate::Resettable for Qne5Spec {
    const RESET_VALUE: u32 = 0;
}
