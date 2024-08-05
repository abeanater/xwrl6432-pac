#[doc = "Register `APPSS_DMA_REQ_SEL` reader"]
pub type R = crate::R<AppssDmaReqSelSpec>;
#[doc = "Register `APPSS_DMA_REQ_SEL` writer"]
pub type W = crate::W<AppssDmaReqSelSpec>;
#[doc = "Field `select` reader - 31:0\\]
Reserved for R&amp;D. Do not touch"]
pub type SelectR = crate::FieldReader<u32>;
#[doc = "Field `select` writer - 31:0\\]
Reserved for R&amp;D. Do not touch"]
pub type SelectW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for R&amp;D. Do not touch"]
    #[inline(always)]
    pub fn select(&self) -> SelectR {
        SelectR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Reserved for R&amp;D. Do not touch"]
    #[inline(always)]
    #[must_use]
    pub fn select(&mut self) -> SelectW<AppssDmaReqSelSpec> {
        SelectW::new(self, 0)
    }
}
#[doc = "APPSS_DMA_REQ_SEL\n\nYou can [`read`](crate::Reg::read) this register and get [`appss_dma_req_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`appss_dma_req_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AppssDmaReqSelSpec;
impl crate::RegisterSpec for AppssDmaReqSelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`appss_dma_req_sel::R`](R) reader structure"]
impl crate::Readable for AppssDmaReqSelSpec {}
#[doc = "`write(|w| ..)` method takes [`appss_dma_req_sel::W`](W) writer structure"]
impl crate::Writable for AppssDmaReqSelSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPSS_DMA_REQ_SEL to value 0"]
impl crate::Resettable for AppssDmaReqSelSpec {
    const RESET_VALUE: u32 = 0;
}
