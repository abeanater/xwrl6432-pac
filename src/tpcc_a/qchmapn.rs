#[doc = "Register `QCHMAPN` reader"]
pub type R = crate::R<QchmapnSpec>;
#[doc = "Register `QCHMAPN` writer"]
pub type W = crate::W<QchmapnSpec>;
#[doc = "Field `TRWORD` reader - 4:2\\]
TRWORD points to the specific trigger word of the PaRAM Entry defined by PAENTRY. A write to the trigger word results in a QDMA Event being recognized."]
pub type TrwordR = crate::FieldReader;
#[doc = "Field `TRWORD` writer - 4:2\\]
TRWORD points to the specific trigger word of the PaRAM Entry defined by PAENTRY. A write to the trigger word results in a QDMA Event being recognized."]
pub type TrwordW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PAENTRY` reader - 13:5\\]
PaRAM Entry number for QDMA Channel N."]
pub type PaentryR = crate::FieldReader<u16>;
#[doc = "Field `PAENTRY` writer - 13:5\\]
PaRAM Entry number for QDMA Channel N."]
pub type PaentryW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `RES10` reader - 31:14\\]
RESERVE FIELD"]
pub type Res10R = crate::FieldReader<u32>;
#[doc = "Field `RES10` writer - 31:14\\]
RESERVE FIELD"]
pub type Res10W<'a, REG> = crate::FieldWriter<'a, REG, 18, u32>;
impl R {
    #[doc = "Bits 2:4 - 4:2\\]
TRWORD points to the specific trigger word of the PaRAM Entry defined by PAENTRY. A write to the trigger word results in a QDMA Event being recognized."]
    #[inline(always)]
    pub fn trword(&self) -> TrwordR {
        TrwordR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:13 - 13:5\\]
PaRAM Entry number for QDMA Channel N."]
    #[inline(always)]
    pub fn paentry(&self) -> PaentryR {
        PaentryR::new(((self.bits >> 5) & 0x01ff) as u16)
    }
    #[doc = "Bits 14:31 - 31:14\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res10(&self) -> Res10R {
        Res10R::new((self.bits >> 14) & 0x0003_ffff)
    }
}
impl W {
    #[doc = "Bits 2:4 - 4:2\\]
TRWORD points to the specific trigger word of the PaRAM Entry defined by PAENTRY. A write to the trigger word results in a QDMA Event being recognized."]
    #[inline(always)]
    #[must_use]
    pub fn trword(&mut self) -> TrwordW<QchmapnSpec> {
        TrwordW::new(self, 2)
    }
    #[doc = "Bits 5:13 - 13:5\\]
PaRAM Entry number for QDMA Channel N."]
    #[inline(always)]
    #[must_use]
    pub fn paentry(&mut self) -> PaentryW<QchmapnSpec> {
        PaentryW::new(self, 5)
    }
    #[doc = "Bits 14:31 - 31:14\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res10(&mut self) -> Res10W<QchmapnSpec> {
        Res10W::new(self, 14)
    }
}
#[doc = "QDMA Channel N Mapping Register\n\nYou can [`read`](crate::Reg::read) this register and get [`qchmapn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`qchmapn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct QchmapnSpec;
impl crate::RegisterSpec for QchmapnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`qchmapn::R`](R) reader structure"]
impl crate::Readable for QchmapnSpec {}
#[doc = "`write(|w| ..)` method takes [`qchmapn::W`](W) writer structure"]
impl crate::Writable for QchmapnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets QCHMAPN to value 0"]
impl crate::Resettable for QchmapnSpec {
    const RESET_VALUE: u32 = 0;
}
