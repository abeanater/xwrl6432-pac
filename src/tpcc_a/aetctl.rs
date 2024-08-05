#[doc = "Register `AETCTL` reader"]
pub type R = crate::R<AetctlSpec>;
#[doc = "Register `AETCTL` writer"]
pub type W = crate::W<AetctlSpec>;
#[doc = "Field `STRTEVT` reader - 5:0\\]
AET Start Event: Dictates the Event Number that will force the tpcc_aet signal to be asserted (high)"]
pub type StrtevtR = crate::FieldReader;
#[doc = "Field `STRTEVT` writer - 5:0\\]
AET Start Event: Dictates the Event Number that will force the tpcc_aet signal to be asserted (high)"]
pub type StrtevtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TYPE` reader - 6:6\\]
AET Event Type: TYPE = 0 : Event specified by STARTEVT applies to DMA Events (set by ER ESR or CER) TYPE = 1 : Event specified by STARTEVT applies to QDMA Events"]
pub type TypeR = crate::BitReader;
#[doc = "Field `TYPE` writer - 6:6\\]
AET Event Type: TYPE = 0 : Event specified by STARTEVT applies to DMA Events (set by ER ESR or CER) TYPE = 1 : Event specified by STARTEVT applies to QDMA Events"]
pub type TypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES66` reader - 7:7\\]
RESERVE FIELD"]
pub type Res66R = crate::BitReader;
#[doc = "Field `RES66` writer - 7:7\\]
RESERVE FIELD"]
pub type Res66W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENDINT` reader - 13:8\\]
AET End Interrupt: Dictates the completion interrupt number that will force the tpcc_aet signal to be deasserted (low)"]
pub type EndintR = crate::FieldReader;
#[doc = "Field `ENDINT` writer - 13:8\\]
AET End Interrupt: Dictates the completion interrupt number that will force the tpcc_aet signal to be deasserted (low)"]
pub type EndintW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `RES65` reader - 30:14\\]
RESERVE FIELD"]
pub type Res65R = crate::FieldReader<u32>;
#[doc = "Field `RES65` writer - 30:14\\]
RESERVE FIELD"]
pub type Res65W<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `EN` reader - 31:31\\]
AET Enable: EN = 0 : AET event generation is disabled. EN = 1 : AET event generation is enabled."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - 31:31\\]
AET Enable: EN = 0 : AET event generation is disabled. EN = 1 : AET event generation is enabled."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - 5:0\\]
AET Start Event: Dictates the Event Number that will force the tpcc_aet signal to be asserted (high)"]
    #[inline(always)]
    pub fn strtevt(&self) -> StrtevtR {
        StrtevtR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bit 6 - 6:6\\]
AET Event Type: TYPE = 0 : Event specified by STARTEVT applies to DMA Events (set by ER ESR or CER) TYPE = 1 : Event specified by STARTEVT applies to QDMA Events"]
    #[inline(always)]
    pub fn type_(&self) -> TypeR {
        TypeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 7:7\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res66(&self) -> Res66R {
        Res66R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - 13:8\\]
AET End Interrupt: Dictates the completion interrupt number that will force the tpcc_aet signal to be deasserted (low)"]
    #[inline(always)]
    pub fn endint(&self) -> EndintR {
        EndintR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 14:30 - 30:14\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res65(&self) -> Res65R {
        Res65R::new((self.bits >> 14) & 0x0001_ffff)
    }
    #[doc = "Bit 31 - 31:31\\]
AET Enable: EN = 0 : AET event generation is disabled. EN = 1 : AET event generation is enabled."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - 5:0\\]
AET Start Event: Dictates the Event Number that will force the tpcc_aet signal to be asserted (high)"]
    #[inline(always)]
    #[must_use]
    pub fn strtevt(&mut self) -> StrtevtW<AetctlSpec> {
        StrtevtW::new(self, 0)
    }
    #[doc = "Bit 6 - 6:6\\]
AET Event Type: TYPE = 0 : Event specified by STARTEVT applies to DMA Events (set by ER ESR or CER) TYPE = 1 : Event specified by STARTEVT applies to QDMA Events"]
    #[inline(always)]
    #[must_use]
    pub fn type_(&mut self) -> TypeW<AetctlSpec> {
        TypeW::new(self, 6)
    }
    #[doc = "Bit 7 - 7:7\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res66(&mut self) -> Res66W<AetctlSpec> {
        Res66W::new(self, 7)
    }
    #[doc = "Bits 8:13 - 13:8\\]
AET End Interrupt: Dictates the completion interrupt number that will force the tpcc_aet signal to be deasserted (low)"]
    #[inline(always)]
    #[must_use]
    pub fn endint(&mut self) -> EndintW<AetctlSpec> {
        EndintW::new(self, 8)
    }
    #[doc = "Bits 14:30 - 30:14\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res65(&mut self) -> Res65W<AetctlSpec> {
        Res65W::new(self, 14)
    }
    #[doc = "Bit 31 - 31:31\\]
AET Enable: EN = 0 : AET event generation is disabled. EN = 1 : AET event generation is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EnW<AetctlSpec> {
        EnW::new(self, 31)
    }
}
#[doc = "Advanced Event Trigger Control\n\nYou can [`read`](crate::Reg::read) this register and get [`aetctl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aetctl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AetctlSpec;
impl crate::RegisterSpec for AetctlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`aetctl::R`](R) reader structure"]
impl crate::Readable for AetctlSpec {}
#[doc = "`write(|w| ..)` method takes [`aetctl::W`](W) writer structure"]
impl crate::Writable for AetctlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets AETCTL to value 0"]
impl crate::Resettable for AetctlSpec {
    const RESET_VALUE: u32 = 0;
}
