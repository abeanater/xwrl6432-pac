#[doc = "Register `PBIST_ALGO` reader"]
pub type R = crate::R<PbistAlgoSpec>;
#[doc = "Register `PBIST_ALGO` writer"]
pub type W = crate::W<PbistAlgoSpec>;
#[doc = "Field `ALGO0` reader - 7:0\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
pub type Algo0R = crate::FieldReader;
#[doc = "Field `ALGO0` writer - 7:0\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
pub type Algo0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ALGO1` reader - 15:8\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
pub type Algo1R = crate::FieldReader;
#[doc = "Field `ALGO1` writer - 15:8\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
pub type Algo1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ALGO2` reader - 23:16\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
pub type Algo2R = crate::FieldReader;
#[doc = "Field `ALGO2` writer - 23:16\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
pub type Algo2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ALGO3` reader - 31:24\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
pub type Algo3R = crate::FieldReader;
#[doc = "Field `ALGO3` writer - 31:24\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
pub type Algo3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
    #[inline(always)]
    pub fn algo0(&self) -> Algo0R {
        Algo0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
    #[inline(always)]
    pub fn algo1(&self) -> Algo1R {
        Algo1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
    #[inline(always)]
    pub fn algo2(&self) -> Algo2R {
        Algo2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 31:24\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
    #[inline(always)]
    pub fn algo3(&self) -> Algo3R {
        Algo3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
    #[inline(always)]
    #[must_use]
    pub fn algo0(&mut self) -> Algo0W<PbistAlgoSpec> {
        Algo0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - 15:8\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
    #[inline(always)]
    #[must_use]
    pub fn algo1(&mut self) -> Algo1W<PbistAlgoSpec> {
        Algo1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - 23:16\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
    #[inline(always)]
    #[must_use]
    pub fn algo2(&mut self) -> Algo2W<PbistAlgoSpec> {
        Algo2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - 31:24\\]
This register is used to indicate the algorithm(s) to be used for the memory self-test routine. Each bit corresponds to a specific algorithm. Writing a value 1 to the particular bit, enables the corresponding algorithm. Writing a value 0 to the particular bit, disables the corresponding algorithm."]
    #[inline(always)]
    #[must_use]
    pub fn algo3(&mut self) -> Algo3W<PbistAlgoSpec> {
        Algo3W::new(self, 24)
    }
}
#[doc = "ROM Algorithm Mask 0\n\nYou can [`read`](crate::Reg::read) this register and get [`pbist_algo::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pbist_algo::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PbistAlgoSpec;
impl crate::RegisterSpec for PbistAlgoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pbist_algo::R`](R) reader structure"]
impl crate::Readable for PbistAlgoSpec {}
#[doc = "`write(|w| ..)` method takes [`pbist_algo::W`](W) writer structure"]
impl crate::Writable for PbistAlgoSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PBIST_ALGO to value 0"]
impl crate::Resettable for PbistAlgoSpec {
    const RESET_VALUE: u32 = 0;
}
